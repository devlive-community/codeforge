//! SSH 隧道：用系统 OpenSSH 的本地端口转发（ssh -L）把目标 DB 端口映射到本地，
//! 由 OpenSSH 负责转发与并发，避免自行实现 SSH 转发的复杂度与原生依赖。
//! 私钥认证用 `ssh -i`；密码认证需本机 `sshpass`；都缺则依赖 ssh-agent/默认密钥。

use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

pub(crate) struct SshConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: Option<String>,
    pub key_file: Option<String>,
}

pub(crate) struct SshTunnel {
    child: Child,
    pub local_port: u16,
}

impl SshTunnel {
    pub fn open(cfg: &SshConfig, target_host: &str, target_port: u16) -> Result<Self, String> {
        let local_port = pick_free_port()?;
        let forward = format!("127.0.0.1:{}:{}:{}", local_port, target_host, target_port);
        let dest = format!("{}@{}", cfg.user, cfg.host);

        let has_key = cfg.key_file.as_deref().is_some_and(|k| !k.is_empty());
        let has_pwd = cfg.password.as_deref().is_some_and(|p| !p.is_empty());

        let mut cmd = if !has_key && has_pwd {
            // 密码认证：经 sshpass 注入密码
            let mut c = Command::new("sshpass");
            c.arg("-p").arg(cfg.password.clone().unwrap_or_default());
            c.arg("ssh");
            c
        } else {
            Command::new("ssh")
        };

        // 非交互：私钥/agent 失败时不要挂起等待输入
        if has_key {
            cmd.arg("-i").arg(cfg.key_file.clone().unwrap_or_default());
            cmd.arg("-o").arg("BatchMode=yes");
        } else if !has_pwd {
            cmd.arg("-o").arg("BatchMode=yes");
        }

        cmd.arg("-N")
            .arg("-T")
            .arg("-o")
            .arg("ExitOnForwardFailure=yes")
            .arg("-o")
            .arg("StrictHostKeyChecking=accept-new")
            .arg("-o")
            .arg("ConnectTimeout=10")
            .arg("-p")
            .arg(cfg.port.to_string())
            .arg("-L")
            .arg(&forward)
            .arg(&dest)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let child = cmd.spawn().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                if !has_key && has_pwd {
                    "SSH 密码认证需要本机安装 sshpass（如 brew install sshpass），或改用私钥认证"
                        .to_string()
                } else {
                    "未找到 ssh 命令，请确认本机已安装 OpenSSH 客户端".to_string()
                }
            } else {
                format!("启动 SSH 隧道失败: {}", e)
            }
        })?;

        let tunnel = SshTunnel { child, local_port };
        tunnel.wait_ready(Duration::from_secs(12))?;
        Ok(tunnel)
    }

    /// 轮询本地转发端口直至可连接，作为隧道就绪信号。
    fn wait_ready(&self, timeout: Duration) -> Result<(), String> {
        let deadline = Instant::now() + timeout;
        loop {
            if TcpStream::connect(("127.0.0.1", self.local_port)).is_ok() {
                return Ok(());
            }
            if Instant::now() >= deadline {
                return Err(
                    "SSH 隧道建立超时（请检查跳板机地址、认证方式与目标端口是否可达）".to_string(),
                );
            }
            std::thread::sleep(Duration::from_millis(150));
        }
    }
}

impl Drop for SshTunnel {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// 取一个空闲本地端口：绑定 :0 拿到端口号后立即释放，交给 ssh 去监听。
fn pick_free_port() -> Result<u16, String> {
    let listener =
        TcpListener::bind("127.0.0.1:0").map_err(|e| format!("分配本地端口失败: {}", e))?;
    let port = listener
        .local_addr()
        .map_err(|e| format!("读取本地端口失败: {}", e))?
        .port();
    Ok(port)
}

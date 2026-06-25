// 常用 .gitignore 模板块（按语言/框架/平台）。每块首行为标题注释，用于去重。
export interface GitignoreTemplate { id: string; label: string; content: string }

export const gitignoreTemplates: GitignoreTemplate[] = [
  {
    id: 'node',
    label: 'Node',
    content: `# Node
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*
.pnpm-store/
dist/
.cache/`
  },
  {
    id: 'python',
    label: 'Python',
    content: `# Python
__pycache__/
*.py[cod]
*.egg-info/
.venv/
venv/
env/
.pytest_cache/
.mypy_cache/
build/
dist/`
  },
  {
    id: 'rust',
    label: 'Rust',
    content: `# Rust
/target/
**/*.rs.bk
Cargo.lock`
  },
  {
    id: 'go',
    label: 'Go',
    content: `# Go
/bin/
/vendor/
*.exe
*.test
*.out
go.work`
  },
  {
    id: 'java',
    label: 'Java / Maven / Gradle',
    content: `# Java
*.class
target/
build/
.gradle/
*.jar
*.war
hs_err_pid*`
  },
  {
    id: 'macos',
    label: 'macOS',
    content: `# macOS
.DS_Store
.AppleDouble
.LSOverride
Icon
._*`
  },
  {
    id: 'windows',
    label: 'Windows',
    content: `# Windows
Thumbs.db
ehthumbs.db
Desktop.ini
$RECYCLE.BIN/
*.lnk`
  },
  {
    id: 'ide',
    label: 'IDE / 编辑器',
    content: `# IDE / Editors
.idea/
.vscode/
*.swp
*.swo
*~`
  },
  {
    id: 'env',
    label: '环境与密钥',
    content: `# Env & secrets
.env
.env.local
.env.*.local
*.pem
*.key`
  },
  {
    id: 'logs',
    label: '日志',
    content: `# Logs
logs/
*.log`
  }
]

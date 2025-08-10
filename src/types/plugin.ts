export default interface PluginConfig
{
    enabled: boolean               // 插件是否启用
    execute_home?: string   // 插件的执行路径
    extension: string      // 插件支持的文件扩展名
    language: string            // 插件所属语言
    before_compile?: string // 插件在编译前执行的命令
    after_compile?: string  // 插件在编译完成后执行的命令
    run_command?: string    // 插件执行的命令，例如 "python2 $filename"
    template?: string       // 插件的模板
}

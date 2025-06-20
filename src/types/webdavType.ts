export type DavConfig ={
  address: string
  port: number
  users: {
    username: string
    password: string
    directory: string
    permissions: string[]
  }[]
}
  
// {
//   address: string // 地址
//   port: number // 端口
//   tls: boolean // 开启TLS
//   cert: string // 证书文件路径
//   key: string // 密钥文件路径
//   prefix: string //  WebDAV路径的默认前缀，所有路径都以这个前缀开始
//   debug: boolean // 调试日志
//   noSniff: boolean //  默认不启用文件嗅探，即不自动检测文件类型。
//   behindProxy: boolean // 服务器不在代理后运行，不使用X-Forwarded-For头来记录远程地址。
//   directory: string // 默认情况下，用户可以访问当前目录。
//   permissions: string | string[] // 默认权限为读取权限 R
//   rules: []
//   rulesBehavior: string // 规则行为默认为覆盖模式，用户规则会覆盖全局规则。
//   log: {
//     format: string // 日志格式为控制台。
//     colors: boolean // 控制台日志启用颜色
//     outputs: string[] // 日志输出
//   }
//   cors: {
//     enabled: boolean // 启用跨域
//     credentials: boolean // 允许凭据
//     allowed_headers: string[] // 允许的头部字段
//     allowed_hosts: string[] // 允许的跨域主机地址
//     allowed_methods: string[] // 允许的HTTP 请求
//     exposed_headers: string[] // 暴露的头部字段
//   }
//   users: {
//     username: string // 用户名
//     password: string // 密码
//     directory?: string // 可访问的路径
//     permissions?: string | string[] // CURD 权限 C | U | R | D | none | CURD
//     rules?: {
//       // 路径规则
//       regex?: string // 匹配的文件名称规则
//       path?: string // 允许访问的路径
//       permissions: string | string[] // CURD 权限 C | U | R | D | none | CURD
//     }[]
//   }[]
// }
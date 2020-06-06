# billing_rust

这是一个用rust语言编写的billing验证服务器，[点击这里](https://github.com/liuguangw/billing_rust/releases)下载我发布的版本。

## 编译方法

```bash
#通用编译命令
cargo build --release
```

在Linux环境下，编译后的项目会依赖`glibc`库。

当然, 也可以选择使用[musl libc](https://musl.cc/)将C库静态链接到最终的可执行文件。

```bash
#musl静态编译命令

#linux X86
cargo build --target=i686-unknown-linux-musl --release

#linux X64
cargo build --target=x86_64-unknown-linux-musl --release
```

## 运行环境要求

如果是`musl`版本,则运行的机器无需任何运行库。

不是`musl`的版本,需要`glibc` 库, 一般linux系统等会附带。但是如果目标服务器的`glibc`版本不同,则有可能导致运行出错。

`rust`编译工具只有在编译的时候需要,运行时则不需要.

## 目标文件说明

```
billing       - Linux版本的billing服务器
billing.exe   - Windows版本的
config.json  - 配置文件
```

## 部署方法

修改`config.json`中的相关配置

```json
{
  "ip": "127.0.0.1",//billing服务器的ip，默认127.0.0.1即可
  "port": 12680,//billing服务器监听的端口(自定义一个未被占用的端口即可)
  "db_host": "127.0.0.1",//MySQL服务器的ip或者主机名
  "db_port": 3306,//MySQL服务器端口
  "db_user": "root",//MySQL用户名
  "db_password": "root",//MySQL密码
  "db_name": "web",//账号数据库名(一般为web)
  "allow_old_password": false,//只有在老版本MySQL报old_password错误时,才需要设置为true
  "auto_reg": true,//用户登录的账号不存在时,是否引导用户进行注册
  "allow_ips": [],//允许的服务端连接ip,为空时表示允许任何ip,不为空时只允许指定的ip连接
  "transfer_number": 1000, //兑换参数，有的版本可能要设置为1才能正常兑换,有的则是1000
    "debug_type": 0 //调试级别: 0/1/2
}
```

> 如果biiling和服务端位于同一台服务器的情况下，建议billing的IP使用127.0.0.1,这样可以避免绕一圈外网
>
> 本项目中附带的`config.json`的各项值为其默认值,如果你的配置中的值与默认值相同,则可以省略
>
> 例如你的配置只有密码和端口和上方配置不同，则可以这样写
>
> {
>
>   "port" : 12681,
>
>   "db_password" : "123456"
>
> }
>
> 如果你的配置和默认配置完全一样，则可以简写为 {}

将`billing` (Windows服务器则是`billing.exe`)和`config.json`放置于同一目录下

修改游戏服务器的配置文件`....../tlbb/Server/Config/ServerInfo.ini`中billing的配置

```ini
#........
[World]
IP=127.0.0.1
Port=777

[Billing]
Number=1
#billing服务器的ip
IP0=127.0.0.1
# billing服务器监听的端口
Port0=12680
#.........
```

最后启动游戏服务端、启动billing即可

## Cli命令选项
```bash
billing up         #Run the billing server in the foreground
billing up -d      #Detached mode: Run the billing server in the background
billing stop       #Stop the billing server
```
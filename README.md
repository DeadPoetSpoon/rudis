<div align="center">

<br />

<img src="./logo/logo.png" height="80"/>

<br />

[Github](https://github.com/sleeprite/rudis) | [Gitee](https://gitee.com/Jmysy/rudis) | [Packages](./release)

<a href='https://gitee.com/rudis/rudis/stargazers'><img src='https://gitee.com/rudis/rudis/badge/star.svg?theme=gvp' alt='star'></img></a>
<a href="https://github.com/sleeprite/rudis"><img src="https://img.shields.io/github/stars/sleeprite/rudis?style=flat-square&logo=GitHub"></a>
<a href="https://github.com/sleeprite/rudis/blob/master/LICENSE"><img src="https://img.shields.io/github/license/sleeprite/rudis.svg?style=flat-square"></a>

<h4>高 性 能 内 存 数 据 库 </h4>

**[🔶 Explore the docs »](https://sleeprite.github.io/rudis)**

</div>

## 项目介绍

Rudis 是一个采用 Rust 语言编写得高性能键值存储系统，旨在利用 Rust 语言的优势来重新复现 Rudis 的核心功能，以满足用户对高性能、可靠性和安全性的需求，同时保证与 Rudis API 的兼容。

### 特性

- 跨平台，兼容 windows、linux 系统架构。
- 兼容 字符串、集合、哈希、列表、有序集合数据结构。
- 提供 rdb 与 aof 机制以支持数据备份和恢复。
- 拥有卓越的处理速度和即时响应能力。
- 兼容 Rudis 的命令和协议规范。

## 快速入门

```
     /\_____/\
    /  o   o  \          Rudis 0.0.1
   ( ==  ^  == )
    )         (          Bind: 127.0.0.1:6379
   (           )
  ( (  )   (  ) )
 (__(__)___(__)__)

[2024-04-30T02:00:55Z INFO  rudis_server] Start loading appendfile
[=======================================] percent: 100% lines: 6/6
[2024-04-30T02:00:55Z INFO  rudis_server] Server initialized
[2024-04-30T02:00:55Z INFO  rudis_server] Ready to accept connections
```

- Cargo 命令

```
// 普通启动
cargo run

// 带参启动
cargo run -- --port 8848
cargo run -- --save 20/1 60/2

// 指定配置
cargo run -- --config rudis.properties

// 构建程序
cargo build

cargo build --release --target=x86_64-unknown-linux-musl

cargo build --release

// 代码检测
cargo clippy
```

## 启动参数

- 配置文件 (config): 指定Rudis配置文件路径。
- 端口 (port): Rudis服务器监听端口，默认6379。
- 绑定的主机地址 (bind): 指定Rudis服务器绑定地址。
- RDB保存策略 (save): 设置RDB自动保存条件。
- 密码 (password): 设置Rudis访问密码。
- 数据库数量 (databases): Rudis数据库数量，默认16。
- 数据持久化目录 (dir): RDB和AOF文件存储目录，默认"./"。
- 持久化日志路径 (appendfilename): AOF日志文件存储路径。
- 开启持久化 (appendonly): 是否开启AOF持久化。
- 数据文件名 (dbfilename): RDB持久化文件名，默认"dump.rdb"。
- 会话上限 (maxclients): 最大客户端连接数，默认1000。
- 定时任务频率 (hz): 定时任务执行频率，默认10次/秒。

## 项目结构

### command

command 包是一个用 Rust 编写的模拟Rudis服务器的组件，主要负责实现Rudis协议的解析、数据库操作的执行以及相关结果的响应。该包内部包含了针对不同Rudis命令的实现，如SELECT、GET、SET等。其核心功能是根据Rudis协议规范，解析来自客户端的命令请求，并在模拟的Rudis数据库上执行相应的操作，再将结果返回给客户端。通过实现各个Rudis命令处理器，实现了对Rudis协议的完整支持，并提供了一个简单而有效的策略来处理不同类型的命令。

### db

db 包是一个基于内存的数据库管理系统。该模块提供了基础的数据结构约定，以及数据库操作功能，包括对数据的增、删、改、查等操作。


### persistence

persistence 模块提供了 AOF（Append-Only File）和 RDB（Rudis Database） 两种持久化机制，它们共同确保了 Rudis 数据库的数据持久性和一致性。AOF 机制通过记录每个写操作并将它们追加到 AOF 文件中，实现了数据的持续更新和完整性。这种机制对于数据的准确性和可靠性至关重要，尤其是在系统故障或重启后能够确保数据的恢复。

### session

session 模块的设计目的是提供一个简单的会话管理功能，用于跟踪用户的操作状态，例如用户所选的数据库索引以及用户是否已认证等信息。这对于需要进行用户认证或者跟踪用户操作状态的系统是非常有用的。

## 常用命令

echo 命令
```
127.0.0.1:6379> echo helloword
helloword
```

ping 命令
```
127.0.0.1:6379> ping
PONG
```

set 命令
```
127.0.0.1:6379> set user bailiang
OK
```

set 命令 [过期]
```
127.0.0.1:6379> set user bailiang px 10000
OK
127.0.0.1:6379> set user bailiang ex 10
OK
```

get 命令
```
127.0.0.1:6379> get user
bailiang
```

del 命令
```
127.0.0.1:6379> del username
(integer) 1
127.0.0.1:6379> del username password
(integer) 2
```

exists 命令
```
127.0.0.1:6379> exists user
(integer) 0
```

keys 命令
```
127.0.0.1:6379> keys *
(empty list or set)
```

auth 命令
```
127.0.0.1:6379> auth 123456
OK
```

expire 命令
```
127.0.0.1:6379> expire user 10000
(integer) 0
```

select 命令
```
127.0.0.1:6379> select 1
OK
```

dbsize 命令
```
127.0.0.1:6379> dbsize
(integer) 2
```

append 命令
```
127.0.0.1:6379> append user bailiang
(integer) 10
```

move 命令
```
127.0.0.1:6379> move user 0
OK
```

rename 命令
```
127.0.0.1:6379> rename username new_username
OK
```

flushdb 命令
```
127.0.0.1:6379> flushdb
OK
```

flushall 命令
```
127.0.0.1:6379> flushall
OK
```

## 命令列表

| Command | Supprt | Appendfile | Test case | Document |
| ------- | ------ | ---------- | --------- |--------- |
| set     | ✅    | ✅         | ✅       | ✅       |
| get     | ✅    | ⚪         | ✅       | ✅       |
| del     | ✅    | ✅         | ✅       | ✅       |
| echo    | ✅    | ⚪         | ⚪       | ⛔       |
| flushdb | ✅    | ✅         | ⛔       | ⛔       |
| flushall| ✅    | ✅         | ⛔       | ⛔       |
| dbsize  | ✅    | ⚪         | ⛔       | ⛔       |
| auth    | ✅    | ⚪         | ⛔       | ⛔       |
| select  | ✅    | ✅         | ⛔       | ⛔       |
| llen    | ✅    | ⚪         | ✅       | ⛔       |
| exists  | ✅    | ⚪         | ✅       | ⛔       |
| expire  | ✅    | ✅         | ✅       | ⛔       |
| rename  | ✅    | ✅         | ✅       | ⛔       |
| move    | ✅    | ✅         | ⛔       | ⛔       |
| lpush   | ✅    | ✅         | ✅       | ⛔       |
| rpush   | ✅    | ✅         | ✅       | ⛔       |
| append  | ✅    | ✅         | ✅       | ⛔       |
| incr    | ✅    | ✅         | ⛔       | ⛔       |
| decr    | ✅    | ✅         | ⛔       | ⛔       |
| lindex  | ✅    | ⚪         | ✅       | ⛔       |
| lpop    | ✅    | ✅         | ⛔       | ⛔       |
| rpop    | ✅    | ✅         | ⛔       | ⛔       |
| lrange  | ✅    | ⚪         | ⛔       | ⛔       |
| ttl     | ✅    | ⚪         | ⛔       | ⛔       |
| pttl    | ✅    | ⚪         | ⛔       | ⛔       |
| type    | ✅    | ⚪         | ⛔       | ⛔       |
| sadd    | ✅    | ✅         | ✅       | ⛔       |
| smembers| ✅    | ⚪         | ✅       | ⛔       |
| scard   | ✅    | ⚪         | ✅       | ⛔       |
| hmset   | ✅    | ✅         | ✅       | ⛔       |
| hget    | ✅    | ⚪         | ✅       | ⛔       |
| hdel    | ✅    | ✅         | ✅       | ⛔       |
| hexists | ✅    | ⚪         | ✅       | ⛔       |
| hset    | ✅    | ✅         | ✅       | ⛔       |
| keys    | ✅    | ⚪         | ✅       | ⛔       |
| zadd    | ✅    | ✅         | ⛔       | ⛔       |
| zscore  | ✅    | ⚪         | ⛔       | ⛔       |
| zcard   | ✅    | ⚪         | ⛔       | ⛔       |
| zcount  | ✅    | ⚪         | ⛔       | ⛔       |
| pexpire | ✅    | ⚪         | ⛔       | ⛔       |
| mset    | ✅    | ✅         | ⛔       | ⛔       |


## 开源共建

Rudis 项目遵循 [GNU GENERAL PUBLIC LICENSE](https://github.com/sleeprite/rudis/blob/master/LICENSE) 开源协议，感谢这些优秀的 [Contributors](https://github.com/sleeprite/rudis/graphs/contributors)。

<a href="https://github.com/sleeprite/rudis/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=sleeprite/rudis" />
</a>

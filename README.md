# cached
simple and fast caching system

简单快速的缓存系统


默认端口号: *9200*

执行过程:
```shell
# 客户端连接成功, 服务端返回 200 状态码
server => client [b"200"]
# 客户端执行 set 命令
client => server [b"set test gaoxiangkang"]
# 服务端返回执行状态码, 成功 0 失败 1
server => client [b"0"]
# 客户端执行 get 命令
client => server [b"get test"]
# 服务端返回执行状态码和结果,以空格做分割, 成功 0 失败 1 
server => client [b"0 gaoxiangkang"]
# 客户端执行 rm 命令
client => server [b"rm test"]
# 服务端返回执行状态码和结果,删除成功后会把删除的value返回, 以空格做分割, 成功 0 失败 1
server => client [b"0 gaoxiangkang"]
```

服务端状态码:
```shell
200  # 连接成功
0    # 执行成功
1    # 执行失败
```

目前支持的功能:
```shell
set key value  # 添加一个缓存, kv键值对形式.
get key        # 根据key获取value
rm  key        # 根据key删除value
```
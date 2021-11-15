# down 一个带进度条的下载工具


- 支持进度条
- 支持指定保存的文件夹
- 支持下载过了不再重复下载
- 支持tar.gz解压


查看帮助
```shell
./down
```
帮助如图：

![](https://img.azhangbaobao.cn/img/20211116010045.png)

下载文件：
```shell
./down https://dldir1.qq.com/weixin/mac/WeChatMac.dmg
```

下载到指定文件夹

```shell
./down https://dldir1.qq.com/weixin/mac/WeChatMac.dmg -t /root
```

下载到当前文件夹并解压

```shell
./down https://hfish.cn-bj.ufileos.com/hfish-2.6.2-linux-amd64.tar.gz --tgz
```

![](https://img.azhangbaobao.cn/img/20211116010522.png)
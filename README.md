1、暂时无法解决 从配置文件中读取服务器信息，在命令行参数带上用户名密码 这两种选项的互斥的参数解析模式。参见：https://github.com/TeXitoi/structopt/issues/141


2、libssh2 rename方法，在跨越根层级目录时会报错的问题。eg: src:"/home/test/dist/assets/theme/0/17.png" dest:"/usr/.trash/travis.json",
这里将dest换为"/home"下的任意存在目录就可成功。
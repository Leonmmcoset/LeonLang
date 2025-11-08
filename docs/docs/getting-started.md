# 快速入门

## 安装

确保您的系统已安装 Rust，然后克隆仓库并构建：

```bash
git clone [仓库地址]
cd LeonLang
cargo build --release
```

## 运行程序

```bash
./target/release/leonlang your_file.leon
```

## 示例程序

创建一个简单的 hello.leon 文件：

```leon
require("basic");
basic.print(string:"Hello from LeonBasic!");
```

运行它：

```bash
./target/release/leonlang hello.leon
```

## 注意事项

* LeonBasic 是一个解释型语言，所有代码都在运行时解析执行
* 变量定义时必须指定类型
* 字符串拼接使用 `+` 运算符
* 函数调用时参数需要指定类型（除了已定义的变量引用）
* 文件操作完成后请记得调用 `close()` 方法关闭文件
* 网络请求可能需要处理超时和错误情况
# 基本语法

## 1. 打印 "Hello world!"

首先引入基础库（Basic）：
```leon
require("basic");
```
然后调用打印函数：
```leon
basic.print(string:"Hello world!");
```
* 格式：basic.print(string: "内容");
* 参数：string 类型，内容为字符串

## 2. 变量

### (1) 利用变量输出 "hi!"
```leon
require("basic");
var(a) = string:"hi!";
basic.print(var(a));
```

### (2) 将 string 和 int 结合为变量 c 并打印
```leon
require("basic");
var(a) = string:"Number: ";
var(b) = int:2;
var(c) = var(a) + string:var(b);
basic.print(var(c));
```
* 输出结果：Number: 2

### (3) 变量类型

支持的类型包括：
* string：字符串
* int：整数
* float：浮点数

## 3. 自定义函数

### (1) 无参数函数
```leon
require("basic");
func(test()) = {
    basic.print("Hello world!");
};
func(test());  // 调用函数，输出 Hello world!
```

### (2) 有参数函数
```leon
require("basic");
func(add(first, second)) = {
    basic.print(int:first + int:second);
};
func(add(int:1, int:2));  // 调用函数，输出 3
```

## 4. 内置库

### (1) basic 库

* print()：打印文字或其他内容
* runoscommand()：运行系统命令
* setrequirepath()：设定 require 库的位置（不影响内置库）
* input()：请求用户输入
* pause()：让用户按下任意键继续

### (2) request 库

* get()：抓取内容
* download()：下载文件
* check()：查看状态码
* header()：查看头信息
* footer()：查看尾信息
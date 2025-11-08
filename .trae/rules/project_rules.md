LeonBasic 编程语言文档

一、基本语法

1. 打印 "Hello world!"

首先引入基础库（Basic）：
require("basic");
然后调用打印函数：
basic.print(string:"Hello world!");
• 格式：basic.print(string: "内容");

• 参数：string 类型，内容为字符串
2. 变量

(1) 利用变量输出 "hi!"
require("basic");
var(a) = string:"hi!";
basic.print(var(a));
(2) 将 string 和 int 结合为变量 c 并打印
require("basic");
var(a) = string:"Number: ";
var(b) = int:2;
var(c) = var(a) + string:var(b);
basic.print(var(c));
• 输出结果：Number: 2

(3) 变量类型

支持的类型包括：

• string：字符串

• int：整数

• float：浮点数
3. 自定义函数

(1) 无参数函数
require("basic");
func(test()) = {
    basic.print("Hello world!");
};
func(test());  // 调用函数，输出 Hello world!
(2) 有参数函数
require("basic");
func(add(first, second)) = {
    basic.print(int:first + int:second);
};
func(add(int:1, int:2));  // 调用函数，输出 3
4. 内置库

(1) basic 库

• print()：打印文字或其他内容

• runoscommand()：运行系统命令

• setrequirepath()：设定 require 库的位置（不影响内置库）

• input()：请求用户输入

• pause()：让用户按下任意键继续

(2) request 库

• get()：抓取内容

• download()：下载文件

• check()：查看状态码

• header()：查看头信息

• footer()：查看尾信息
二、控制流

1. 条件语句（if-else）
require("basic");
var(a) = int:5;
if(a > 3) {
    basic.print("a 大于 3");
} else {
    basic.print("a 不大于 3");
}
2. 循环（for/while）
require("basic");

// for 循环
for(var(i) = int:0; i < 5; i = i + 1) {
    basic.print(string:"循环次数: " + string:i);
}

// while 循环
var(j) = int:0;
while(j < 3) {
    basic.print(string:"while 循环: " + string:j);
    j = j + 1;
}
三、文件操作

1. 文件写入
require("basic");
var(file) = basic.open("test.txt", "write");
basic.write(file, "Hello, world!");
basic.close(file);
2. 文件读取
require("basic");
var(file) = basic.open("test.txt", "read");
var(content) = basic.read(file);
basic.print(content);
basic.close(file);
3. 文件追加
require("basic");
var(file) = basic.open("test.txt", "append");
basic.write(file, " 追加内容");
basic.close(file);
四、网络功能

1. HTTP GET 请求
require("request");
var(response) = request.get("https://example.com");
basic.print(response);
2. 下载文件
require("request");
request.download("https://example.com/image.jpg", "local.jpg");
五、错误处理

1. try-catch 异常捕获
require("basic");
try {
    var(a) = int:5 / int:0;
} catch(error) {
    basic.print(string:"发生错误: " + error);
}
六、用户交互

1. 菜单系统
require("basic");
basic.print("1. 选项一");
basic.print("2. 选项二");
basic.print("3. 退出");
var(choice) = basic.input("请选择: ");
if(choice == "1") {
    basic.print("你选择了选项一");
} else if(choice == "2") {
    basic.print("你选择了选项二");
} else {
    basic.print("退出程序");
}

---

注：
这只是一个示例，你也可以编写其他的功能等等。

所有的LeonBasic文档放在/docs目录下，为Markdown格式，MKDocs。
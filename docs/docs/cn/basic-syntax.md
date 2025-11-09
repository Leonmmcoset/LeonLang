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

## 4. 转义字符

在字符串中，可以使用转义字符来表示特殊字符：

### (1) 支持的转义字符

* `\n`：换行符
* `\t`：制表符
* `\r`：回车符
* `\"`：双引号
* `\\`：反斜杠
* `\+`：加号（特殊转义，显示为+）

### (2) 转义字符示例

```leon
require("basic");

// 基本转义字符
basic.print(string:"第一行\n第二行");

// 转义双引号
basic.print(string:"他说：\"你好！\"");

// \+转义为+
basic.print(string:"5 \+ 3 = 8");

// 反斜杠转义
basic.print(string:"路径：C:\\\\folder\\\\file.txt");
```

### (3) 转义字符在字符串拼接中的使用

```leon
require("basic");
var(a) = string:"Hello ";
var(b) = string:"\+ World!";
basic.print(var(a) + var(b));  // 输出：Hello + World!

## 5. 自定义函数

### (1) 无参数函数
```leon
require("basic");
func(test()) = {
    basic.print("Hello world!");
};
test();  // 调用函数，输出 Hello world!
```

### (2) 有参数函数
```leon
require("basic");
func(add(self(first), self(second))) = {
    basic.print(int:self(first) + int:self(second));
};
add(int:1, int:2);  // 调用函数，输出 3
```

### (3) 参数访问语法

在函数体中，使用 `self(param_name)` 来访问参数值：
* `int:self(first)` - 获取参数 first 的整数值
* `string:self(second)` - 获取参数 second 的字符串值
* `float:self(param)` - 获取参数 param 的浮点数值

## 6. 内置库

### (1) basic 库

* `print()`：打印文字或其他内容
* `runoscommand()`：运行系统命令
* `setrequirepath()`：设定 require 库的位置（不影响内置库）
* `input()`：请求用户输入
* `pause()`：让用户按下任意键继续

### (2) request 库

* `get()`：抓取内容
* `download()`：下载文件
* `check()`：查看状态码
* `header()`：查看头信息
* `footer()`：查看尾信息

### (3) time 库

* `timestamp()`：获取当前时间戳（毫秒）
* `formatTime()`：格式化时间戳为字符串
* `sleep()`：程序暂停指定毫秒数
* `diffTime()`：计算两个时间戳之间的差值（毫秒）
* `getDateTime()`：获取当前日期和时间字符串

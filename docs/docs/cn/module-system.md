# 模块系统

## 1. require 函数

LeonBasic 提供了 `require()` 函数用于导入内置库和自定义模块，实现代码的模块化和复用。

### (1) 导入内置库

```leon
// 导入基础库
require("basic");

// 导入网络请求库
require("request");

// 导入时间处理库
require("time");
```

### (2) 导入自定义模块

```leon
// 导入当前目录下的 utils.leon 模块
require("utils");

// 使用模块中的函数
var(result) = utils.add(int:5, int:3);
basic.print(string:"5 + 3 = " + string:var(result));
```

## 2. 模块查找路径

当使用 `require(module_name)` 时，系统会按以下顺序查找模块：

1. 首先查找内置库（如 basic、request）
2. 然后在当前脚本所在目录查找 `module_name.leon` 文件
3. 如果未找到，会在当前目录下的 `lib` 子目录查找

## 3. 自定义模块创建

### (1) 创建简单模块

创建一个名为 `utils.leon` 的文件：

```leon
// utils.leon

// 定义模块函数
func(add(a, b)) = {
    return a + b;
};

func(multiply(a, b)) = {
    return a * b;
};

func(greet(name)) = {
    return "Hello, " + name + "!";
};
```

### (2) 模块使用示例

```leon
// main.leon
require("basic");
require("utils");

// 使用 utils 模块中的函数
var(sum) = utils.add(int:5, int:3);
basic.print(string:"5 + 3 = " + string:var(sum));

var(product) = utils.multiply(int:7, int:4);
basic.print(string:"7 * 4 = " + string:var(product));

var(greeting) = utils.greet(string:"World");
basic.print(var(greeting));
```

## 4. 设置 require 路径

可以使用 `basic.setrequirepath()` 函数自定义模块查找路径：

```leon
require("basic");

// 设置自定义模块路径
basic.setrequirepath("D:\\Projects\\LeonBasic\\modules");

// 现在可以从自定义路径加载模块
require("custom_module");
```

## 5. 模块函数调用语法

使用导入的模块函数时，遵循以下语法：

```leon
// 模块名.函数名(参数...)
module_name.function_name(param1, param2, ...);
```

## 6. 模块加载注意事项

* 模块文件必须以 `.leon` 为扩展名
* 一个模块可以被多次导入，但只会执行一次
* 模块中的变量和函数对导入者可见
* 模块之间可以相互导入，但要避免循环依赖
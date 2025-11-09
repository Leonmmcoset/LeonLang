# 时间处理库 (time)

## 1. 介绍

LeonBasic 提供了 `time` 内置库，用于处理时间相关的操作，包括获取时间戳、格式化时间、程序暂停等功能。

## 2. 导入方法

在使用 `time` 库之前，需要先导入：

```leon
require("time");
```

## 3. 核心函数

### (1) timestamp() - 获取当前时间戳

获取当前系统时间的毫秒时间戳。

**参数**：无
**返回值**：整数类型，表示从1970-01-01 00:00:00 UTC开始的毫秒数

**示例**：
```leon
require("basic");
require("time");

// 获取当前时间戳
var(ts) = time.timestamp();
basic.print(string:"当前时间戳: " + string:var(ts));
```

### (2) formatTime(timestamp, format) - 格式化时间戳

将时间戳格式化为指定格式的字符串。

**参数**：
- timestamp: 整数类型，时间戳
- format: 字符串类型，格式字符串

**返回值**：字符串类型，格式化后的时间字符串

**格式说明**：
- `yyyy`: 四位数年份
- `MM`: 两位数月份 (01-12)
- `dd`: 两位数日期 (01-31)
- `HH`: 两位数小时 (00-23)
- `mm`: 两位数分钟 (00-59)
- `ss`: 两位数秒数 (00-59)
- `SSS`: 三位数毫秒 (000-999)

**示例**：
```leon
require("basic");
require("time");

// 获取当前时间戳
var(ts) = time.timestamp();

// 默认格式 (yyyy-MM-dd HH:mm:ss)
var(default_format) = time.formatTime(var(ts), "");
basic.print(string:"默认格式: " + var(default_format));

// 自定义格式
var(custom_format) = time.formatTime(var(ts), "yyyy/MM/dd HH-mm-ss.SSS");
basic.print(string:"自定义格式: " + var(custom_format));
```

### (3) sleep(milliseconds) - 程序暂停

使程序暂停执行指定的毫秒数。

**参数**：
- milliseconds: 整数类型，暂停的毫秒数

**返回值**：无

**示例**：
```leon
require("basic");
require("time");

basic.print("开始等待...");

// 暂停2000毫秒（2秒）
time.sleep(int:2000);

basic.print("等待结束！");
```

### (4) diffTime(start_time, end_time) - 计算时间差

计算两个时间戳之间的差值（毫秒）。

**参数**：
- start_time: 整数类型，开始时间戳
- end_time: 整数类型，结束时间戳

**返回值**：整数类型，时间差（毫秒）

**示例**：
```leon
require("basic");
require("time");

// 记录开始时间
var(start) = time.timestamp();

// 执行一些操作
var(sum) = int:0;
for(var(i) = int:0; i < int:1000000; i = i + int:1) {
    sum = sum + i;
}

// 记录结束时间
var(end) = time.timestamp();

// 计算执行时间
var(duration) = time.diffTime(var(start), var(end));
basic.print(string:"操作执行时间: " + string:var(duration) + " 毫秒");
```

### (5) getDateTime() - 获取当前日期时间

获取当前系统的日期和时间字符串。

**参数**：无
**返回值**：字符串类型，当前日期和时间（格式：yyyy-MM-dd HH:mm:ss）

**示例**：
```leon
require("basic");
require("time");

// 获取当前日期时间
var(now) = time.getDateTime();
basic.print(string:"当前时间: " + var(now));
```

## 4. 综合示例

### 4.1 倒计时器

```leon
require("basic");
require("time");

// 倒计时秒数
var(countdown) = int:10;

basic.print(string:"倒计时 " + string:var(countdown) + " 秒开始...");

// 倒计时循环
while(var(countdown) > int:0) {
    basic.print(string:"剩余时间: " + string:var(countdown) + " 秒");
    time.sleep(int:1000);  // 暂停1秒
    countdown = countdown - int:1;
}

basic.print("倒计时结束！");
```

### 4.2 性能计时器

```leon
require("basic");
require("time");

// 测试不同操作的性能
basic.print("性能测试开始...");

// 测试字符串拼接
var(str_start) = time.timestamp();
var(long_str) = string:"";
for(var(i) = int:0; i < int:10000; i = i + int:1) {
    long_str = long_str + string:"a";
}
var(str_end) = time.timestamp();
var(str_time) = time.diffTime(var(str_start), var(str_end));
basic.print(string:"字符串拼接10000次耗时: " + string:var(str_time) + " 毫秒");

// 测试数学运算
var(math_start) = time.timestamp();
var(result) = int:0;
for(var(i) = int:0; i < int:10000000; i = i + int:1) {
    result = (result + i) % int:100;
}
var(math_end) = time.timestamp();
var(math_time) = time.diffTime(var(math_start), var(math_end));
basic.print(string:"数学运算1000万次耗时: " + string:var(math_time) + " 毫秒");
```

## 5. 注意事项

1. 时间戳是基于系统时间的，用户修改系统时间会影响时间戳的准确性。
2. `sleep()` 函数会阻塞程序执行，在执行过程中程序将暂停响应。
3. 格式化时间的字符串格式不区分大小写，但建议使用大写格式以提高可读性。
4. 时间戳的精度为毫秒级，适用于大多数应用场景。
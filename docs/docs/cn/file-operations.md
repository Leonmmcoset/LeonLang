# 文件操作

## 1. 文件写入
```leon
require("basic");
var(file) = basic.open("test.txt", "write");
basic.write(file, "Hello, world!");
basic.close(file);
```

## 2. 文件读取
```leon
require("basic");
var(file) = basic.open("test.txt", "read");
var(content) = basic.read(file);
basic.print(content);
basic.close(file);
```

## 3. 文件追加
```leon
require("basic");
var(file) = basic.open("test.txt", "append");
basic.write(file, " 追加内容");
basic.close(file);
```
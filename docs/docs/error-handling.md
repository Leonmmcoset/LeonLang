# 错误处理

## 1. try-catch 异常捕获
```leon
require("basic");
try {
    var(a) = int:5 / int:0;
} catch(error) {
    basic.print(string:"发生错误: " + error);
}
```
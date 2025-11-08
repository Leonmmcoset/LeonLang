# 控制流

## 1. 条件语句（if-else）
```leon
require("basic");
var(a) = int:5;
if(a > 3) {
    basic.print("a 大于 3");
} else {
    basic.print("a 不大于 3");
}
```

## 2. 循环

### (1) for 循环
```leon
require("basic");
for(var(i) = int:0; i < 5; i = i + 1) {
    basic.print(string:"循环次数: " + string:i);
}
```

### (2) while 循环
```leon
require("basic");
var(j) = int:0;
while(j < 3) {
    basic.print(string:"while 循环: " + string:j);
    j = j + 1;
}
```
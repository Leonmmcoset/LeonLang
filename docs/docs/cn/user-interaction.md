# 用户交互

## 1. 菜单系统
```leon
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
```
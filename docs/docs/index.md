# 欢迎使用 LeonBasic 编程语言

LeonBasic 是一种简单易用的解释型编程语言，专为初学者和快速原型开发设计。它提供了简洁明了的语法，同时支持基本的编程功能，如变量、函数、控制流、文件操作和网络功能等。

这是他的运行原理图：

```mermaid
graph LR
    %% 外部参与者
    Dev["开发者"]:::external

    subgraph "核心引擎"
        direction TB
        CLI["LeonBasic 命令行工具"]:::core
        Parser["解析器与抽象语法树"]:::core
        Evaluator["执行器 / 运行时"]:::core
        Builtins["内置库"]:::core
        PackageLoader["包/模块加载器"]:::core
    end

    TestRunner["测试工具集"]:::test

    subgraph "工具与集成"
        direction TB
        VSExt["VS Code 扩展"]:::tooling
        Docs["文档生成器"]:::tooling
        Trae["Trae 引擎"]:::tooling
    end

    %% 流程（优化箭头文本空格+逻辑表述）
    Dev -->|"编写 .leon 代码"| CLI
    CLI -->|"解析并构建抽象语法树"| Parser
    Parser -->|"将抽象语法树传递给"| Evaluator
    Evaluator -->|"调用函数"| Builtins
    Evaluator -->|"解析模块"| PackageLoader

    TestRunner -->|"通过命令行工具运行 .leon 代码"| CLI

    Dev -->|"编辑与预览"| VSExt
    Dev -->|"编写文档"| Docs
    Dev -->|"配置项目/持续集成"| Trae

    %% 点击事件（链接保持原地址，方便跳转源码）
    click CLI "https://github.com/leonmmcoset/leonlang/blob/master/src/main.rs"
    click Parser "https://github.com/leonmmcoset/leonlang/blob/master/src/main.rs"
    click Evaluator "https://github.com/leonmmcoset/leonlang/blob/master/src/main.rs"
    click Builtins "https://github.com/leonmmcoset/leonlang/blob/master/src/builtins/mod.rs"
    click PackageLoader "https://github.com/leonmmcoset/leonlang/blob/master/src/package/mod.rs"
    click TestRunner "https://github.com/leonmmcoset/leonlang/tree/master/test/"
    click VSExt "https://github.com/leonmmcoset/leonlang/tree/master/addons/"
    click Docs "https://github.com/leonmmcoset/leonlang/tree/master/docs/"
    click Trae "https://github.com/leonmmcoset/leonlang/tree/master/.trae/"

    %% 样式（优化：加结尾分号）
    classDef core fill:#a8d0e6,stroke:#000,stroke-width:1px;
    classDef tooling fill:#b0e57c,stroke:#000,stroke-width:1px;
    classDef test fill:#f4a261,stroke:#000,stroke-width:1px;
    classDef external fill:#ccc,stroke:#000,stroke-width:1px,stroke-dasharray: 5 5;
```

## 主要特性

* **简洁的语法** - 易于学习和使用
* **内置库支持** - 包含基础操作和网络功能
* **灵活的变量系统** - 支持字符串、整数和浮点数
* **完善的控制流** - 包括条件语句和循环结构
* **文件操作** - 支持文件的读写和追加
* **网络功能** - 支持 HTTP 请求和文件下载
* **错误处理** - 提供异常捕获机制

## 快速开始

1. 查看[快速入门](getting-started.md)指南
2. 学习[基本语法](basic-syntax.md)
3. 探索[控制流](control-flow.md)、[文件操作](file-operations.md)等高级功能

## 文档导航

使用左侧导航栏浏览完整文档，包括：

* [基本语法](basic-syntax.md) - 变量、函数和内置库
* [控制流](control-flow.md) - 条件语句和循环
* [文件操作](file-operations.md) - 读写文件
* [网络功能](network.md) - HTTP 请求和下载
* [错误处理](error-handling.md) - 异常捕获
* [用户交互](user-interaction.md) - 菜单和用户输入
* [快速入门](getting-started.md) - 安装和使用指南

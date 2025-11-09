# Welcome to LeonBasic Programming Language

LeonBasic is a simple and easy-to-use interpreted programming language designed for beginners and rapid prototyping. It provides clear and concise syntax while supporting basic programming features such as variables, functions, control flow, file operations, and network capabilities.

Here's its operational diagram:

```mermaid
graph LR
    %% External participants
    Dev["Developer"]:::external

    subgraph "Core Engine"
        direction TB
        CLI["LeonBasic Command Line Tool"]:::core
        Parser["Parser & Abstract Syntax Tree"]:::core
        Evaluator["Executor / Runtime"]:::core
        Builtins["Built-in Libraries"]:::core
        PackageLoader["Package/Module Loader"]:::core
    end

    TestRunner["Testing Tools"]:::test

    subgraph "Tools & Integration"
        direction TB
        VSExt["VS Code Extension"]:::tooling
        Docs["Documentation Generator"]:::tooling
        Trae["Trae Engine"]:::tooling
    end

    %% Flow (improved arrow text with spaces + logical description)
    Dev -->|"Write .leon code"| CLI
    CLI -->|"Parse and build AST"| Parser
    Parser -->|"Pass AST to"| Evaluator
    Evaluator -->|"Call functions"| Builtins
    Evaluator -->|"Parse modules"| PackageLoader

    TestRunner -->|"Run .leon code via CLI"| CLI

    Dev -->|"Edit and preview"| VSExt
    Dev -->|"Write documentation"| Docs
    Dev -->|"Configure project/CI"| Trae

    %% Click events (links remain original for easy source code navigation)
    click CLI "https://github.com/leonmmcoset/leonlang/blob/master/src/main.rs"
    click Parser "https://github.com/leonmmcoset/leonlang/blob/master/src/main.rs"
    click Evaluator "https://github.com/leonmmcoset/leonlang/blob/master/src/main.rs"
    click Builtins "https://github.com/leonmmcoset/leonlang/blob/master/src/builtins/mod.rs"
    click PackageLoader "https://github.com/leonmmcoset/leonlang/blob/master/src/package/mod.rs"
    click TestRunner "https://github.com/leonmmcoset/leonlang/tree/master/test/"
    click VSExt "https://github.com/leonmmcoset/leonlang/tree/master/addons/"
    click Docs "https://github.com/leonmmcoset/leonlang/tree/master/docs/"
    click Trae "https://github.com/leonmmcoset/leonlang/tree/master/.trae/"

    %% Styles (improved: added trailing semicolon)
    classDef core fill:#a8d0e6,stroke:#000,stroke-width:1px;
    classDef tooling fill:#b0e57c,stroke:#000,stroke-width:1px;
    classDef test fill:#f4a261,stroke:#000,stroke-width:1px;
    classDef external fill:#ccc,stroke:#000,stroke-width:1px,stroke-dasharray: 5 5;
```

## Main Features

* **Concise syntax** - Easy to learn and use
* **Built-in libraries** - Including basic operations and network capabilities
* **Flexible variable system** - Supports strings, integers, and floating-point numbers
* **Comprehensive control flow** - Including conditional statements and loops
* **File operations** - Supports reading, writing, and appending to files
* **Network functionality** - Supports HTTP requests and file downloads
* **Error handling** - Provides exception catching mechanisms

## Quick Start

1. Check out the [Getting Started](getting-started.md) guide
2. Learn the [Basic Syntax](basic-syntax.md)
3. Explore advanced features like [Control Flow](control-flow.md), [File Operations](file-operations.md), and more

## Documentation Navigation

Use the left navigation bar to browse the complete documentation, including:

* [Getting Started](getting-started.md) - Installation and usage guide
* [Basic Syntax](basic-syntax.md) - Variables, functions, escape characters, and built-in libraries
* [Module System](module-system.md) - require function and module importing
* [Control Flow](control-flow.md) - Conditional statements and loops
* [File Operations](file-operations.md) - Reading and writing files
* [Network](network.md) - HTTP requests and downloads
* [Error Handling](error-handling.md) - Exception catching
* [User Interaction](user-interaction.md) - Menus and user input
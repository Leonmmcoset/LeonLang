# Getting Started with LeonBasic

## Installation

LeonBasic is a Rust-based interpreted language. Follow these steps to install and run it:

### Prerequisites

- Rust programming language (latest stable version recommended)
- Cargo package manager (included with Rust)

### Clone the Repository

```bash
git clone https://github.com/leonmmcoset/leonlang.git
cd leonlang
```

### Build the Project

```bash
cargo build
```

### Run a LeonBasic Program

```bash
cargo run path/to/your/file.leon
```

## Your First LeonBasic Program

Create a file named `hello.leon` with the following content:

```leon
require("basic");
basic.print(string:"Hello, World!");
```

Run it with:

```bash
cargo run hello.leon
```

You should see the output: `Hello, World!`

## Basic Workflow

1. **Write your code** in a `.leon` file
2. **Run the code** using the LeonBasic interpreter
3. **Debug** as needed

## Development Tools

### VS Code Extension

LeonBasic provides a VS Code extension for better development experience. You can find it in the `addons` directory of the repository.

To install the extension:

1. Open VS Code
2. Go to Extensions
3. Click on the three dots in the top right corner
4. Select "Install from VSIX..."
5. Choose the `.vsix` file from the `addons` directory

## Next Steps

- Learn about [Basic Syntax](basic-syntax.md)
- Explore the [Module System](module-system.md)
- Check out examples in the `test` directory
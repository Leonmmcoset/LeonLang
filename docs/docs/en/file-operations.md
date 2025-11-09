# File Operations

LeonBasic provides functions for basic file operations, including reading, writing, and appending to files.

## 1. File Writing

```leon
require("basic");
var(file) = basic.open("test.txt", "write");
basic.write(file, "Hello, world!");
basic.close(file);
```

## 2. File Reading

```leon
require("basic");
var(file) = basic.open("test.txt", "read");
var(content) = basic.read(file);
basic.print(content);
basic.close(file);
```

## 3. File Appending

```leon
require("basic");
var(file) = basic.open("test.txt", "append");
basic.write(file, " Append content");
basic.close(file);
```

## 4. File Open Modes

LeonBasic supports the following file open modes:

* `"read"` - Open file for reading
* `"write"` - Open file for writing (overwrites existing content)
* `"append"` - Open file for appending (adds to existing content)

## 5. Error Handling in File Operations

It's good practice to handle potential errors when working with files:

```leon
require("basic");
try {
    var(file) = basic.open("nonexistent.txt", "read");
    var(content) = basic.read(file);
    basic.print(content);
    basic.close(file);
} catch(error) {
    basic.print(string:"Error: " + error);
}
```

## 6. Working with File Paths

```leon
require("basic");

// Using absolute path
var(file) = basic.open("C:\\Projects\\test.txt", "write");
basic.write(file, "Content with absolute path");
basic.close(file);

// Using relative path
var(file2) = basic.open(".\\data\\info.txt", "write");
basic.write(file2, "Content with relative path");
basic.close(file2);
```

## 7. Reading Line by Line

While LeonBasic doesn't have a built-in function for reading line by line, you can split the content by newline characters:

```leon
require("basic");

// This is a simplified example that assumes the file exists
var(file) = basic.open("test.txt", "read");
var(content) = basic.read(file);
basic.close(file);

// Note: This is just conceptual - actual implementation would require string splitting functionality
// which may need to be implemented based on LeonBasic's capabilities
```
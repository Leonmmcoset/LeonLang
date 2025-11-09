# Basic Syntax

## 1. Printing "Hello world!"

First, import the basic library:
```leon
require("basic");
```
Then call the print function:
```leon
basic.print(string:"Hello world!");
```
* Format: basic.print(string: "content");
* Parameter: string type, the content to be printed

## 2. Variables

### (1) Using variables to output "hi!"
```leon
require("basic");
var(a) = string:"hi!";
basic.print(var(a));
```

### (2) Combining string and int into variable c and printing
```leon
require("basic");
var(a) = string:"Number: ";
var(b) = int:2;
var(c) = var(a) + string:var(b);
basic.print(var(c));
```
* Output result: Number: 2

### (3) Variable Types

Supported types include:
* string: string
* int: integer
* float: floating-point number

## 3. Escape Characters

In strings, you can use escape characters to represent special characters:

### (1) Supported Escape Characters

* `\n`: newline character
* `\t`: tab character
* `\r`: carriage return
* `\"`: double quote
* `\\`: backslash
* `\+`: plus sign (special escape, displays as +)

### (2) Escape Character Examples

```leon
require("basic");

// Basic escape characters
basic.print(string:"First line\nSecond line");

// Escape double quotes
basic.print(string:"He said: \"Hello!\"");

// \+ escapes to +
basic.print(string:"5 \+ 3 = 8");

// Backslash escape
basic.print(string:"Path: C:\\\\folder\\\\file.txt");
```

### (3) Using Escape Characters in String Concatenation

```leon
require("basic");
var(a) = string:"Hello ";
var(b) = string:"\+ World!";
basic.print(var(a) + var(b));  // Output: Hello + World!
```

## 4. Custom Functions

### (1) Functions Without Parameters
```leon
require("basic");
func(test()) = {
    basic.print("Hello world!");
};
test();  // Call function, outputs Hello world!
```

### (2) Functions With Parameters
```leon
require("basic");
func(add(self(first), self(second))) = {
    basic.print(int:self(first) + int:self(second));
};
add(int:1, int:2);  // Call function, outputs 3
```

### (3) Parameter Access Syntax

In function bodies, use `self(param_name)` to access parameter values:
* `int:self(first)` - Get the integer value of parameter first
* `string:self(second)` - Get the string value of parameter second
* `float:self(param)` - Get the floating-point value of parameter param

## 5. Built-in Libraries

### (1) basic library

* print(): Print text or other content
* runoscommand(): Run system commands
* setrequirepath(): Set the location of require libraries (doesn't affect built-in libraries)
* input(): Request user input
* pause(): Let the user press any key to continue

### (2) request library

* get(): Fetch content
* download(): Download files
* check(): View status code
* header(): View header information
* footer(): View footer information
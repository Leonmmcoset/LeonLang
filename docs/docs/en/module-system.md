# Module System

## 1. require Function

LeonBasic provides the `require()` function for importing built-in libraries and custom modules, enabling code modularity and reuse.

### (1) Importing Built-in Libraries

```leon
// Import basic library
require("basic");

// Import network request library
require("request");
```

### (2) Importing Custom Modules

```leon
// Import utils.leon module from the current directory
require("utils");

// Use functions from the module
var(result) = utils.add(int:5, int:3);
basic.print(string:"5 + 3 = " + string:var(result));
```

## 2. Module Search Path

When using `require(module_name)`, the system will search for modules in the following order:

1. First, look for built-in libraries (such as basic, request)
2. Then look for `module_name.leon` file in the current script's directory
3. If not found, it will look in the `lib` subdirectory of the current directory

## 3. Creating Custom Modules

### (1) Creating a Simple Module

Create a file named `utils.leon`:

```leon
// utils.leon

// Define module functions
func(add(a, b)) = {
    return a + b;
};

func(multiply(a, b)) = {
    return a * b;
};

func(greet(name)) = {
    return "Hello, " + name + "!";
};
```

### (2) Module Usage Example

```leon
// main.leon
require("basic");
require("utils");

// Use functions from the utils module
var(sum) = utils.add(int:5, int:3);
basic.print(string:"5 + 3 = " + string:var(sum));

var(product) = utils.multiply(int:7, int:4);
basic.print(string:"7 * 4 = " + string:var(product));

var(greeting) = utils.greet(string:"World");
basic.print(var(greeting));
```

## 4. Setting require Path

You can use the `basic.setrequirepath()` function to customize the module search path:

```leon
require("basic");

// Set custom module path
basic.setrequirepath("C:\\Projects\\LeonBasic\\modules");

// Now you can load modules from the custom path
require("custom_module");
```

## 5. Module Function Call Syntax

When using imported module functions, follow this syntax:

```leon
// module_name.function_name(param1, param2, ...)
module_name.function_name(param1, param2, ...);
```

## 6. Module Loading Notes

* Module files must have the `.leon` extension
* A module can be imported multiple times but will only be executed once
* Variables and functions in a module are visible to the importer
* Modules can import each other, but avoid circular dependencies
# User Interaction

LeonBasic provides functions for interacting with users, including requesting input and creating simple menu systems.

## 1. Menu System

```leon
require("basic");
basic.print("1. Option One");
basic.print("2. Option Two");
basic.print("3. Exit");
var(choice) = basic.input("Please select: ");
if(choice == "1") {
    basic.print("You selected Option One");
} else if(choice == "2") {
    basic.print("You selected Option Two");
} else {
    basic.print("Exiting program");
}
```

## 2. Getting User Input

Use the `basic.input()` function to request input from the user:

```leon
require("basic");
var(name) = basic.input("What is your name? ");
basic.print(string:"Hello, " + var(name) + string:"!");
```

## 3. Input Validation

```leon
require("basic");
var(age_input) = basic.input("Please enter your age: ");
try {
    var(age) = int:var(age_input);
    if(age > 0) {
        basic.print(string:"You are " + string:age + string:" years old");
    } else {
        basic.print("Invalid age entered");
    }
} catch(error) {
    basic.print("Please enter a valid number");
}
```

## 4. Multi-step User Interaction

```leon
require("basic");
basic.print("--- User Information Form ---");

var(first_name) = basic.input("First Name: ");
var(last_name) = basic.input("Last Name: ");
var(email) = basic.input("Email: ");

basic.print("");
basic.print("Your information:");
basic.print(string:"Name: " + var(first_name) + string:" " + var(last_name));
basic.print(string:"Email: " + var(email));
```

## 5. Confirmation Dialog

```leon
require("basic");
var(confirm) = basic.input("Are you sure you want to continue? (y/n): ");
if(confirm == "y" || confirm == "Y") {
    basic.print("Continuing with the operation...");
} else {
    basic.print("Operation cancelled");
}
```

## 6. Using pause() for User Experience

The `pause()` function can be used to create better user experience by waiting for user confirmation before proceeding:

```leon
require("basic");
basic.print("Important information displayed here");
basic.print("Press any key to continue...");
basic.pause();
basic.print("Continuing with the program");
```

## 7. Interactive Calculator Example

```leon
require("basic");
basic.print("--- Simple Calculator ---");
var(num1_input) = basic.input("Enter first number: ");
var(num2_input) = basic.input("Enter second number: ");
var(operation) = basic.input("Select operation (+, -, *, /): ");

try {
    var(num1) = int:var(num1_input);
    var(num2) = int:var(num2_input);
    
    if(operation == "+ ") {
        basic.print(string:"Result: " + string:(num1 + num2));
    } else if(operation == "- ") {
        basic.print(string:"Result: " + string:(num1 - num2));
    } else if(operation == "* ") {
        basic.print(string:"Result: " + string:(num1 * num2));
    } else if(operation == "/ ") {
        basic.print(string:"Result: " + string:(num1 / num2));
    } else {
        basic.print("Invalid operation");
    }
} catch(error) {
    basic.print(string:"Error: " + error);
}
```
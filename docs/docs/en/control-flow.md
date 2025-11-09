# Control Flow

## 1. Conditional Statements (if-else)

```leon
require("basic");
var(a) = int:5;
if(a > 3) {
    basic.print("a is greater than 3");
} else {
    basic.print("a is not greater than 3");
}
```

## 2. Loops (for/while)

```leon
require("basic");

// for loop
for(var(i) = int:0; i < 5; i = i + 1) {
    basic.print(string:"Loop count: " + string:i);
}

// while loop
var(j) = int:0;
while(j < 3) {
    basic.print(string:"While loop: " + string:j);
    j = j + 1;
}
```

## 3. Nested Conditional Statements

```leon
require("basic");
var(score) = int:85;

if(score >= 90) {
    basic.print("Excellent!");
} else if(score >= 80) {
    basic.print("Good!");
} else if(score >= 60) {
    basic.print("Pass.");
} else {
    basic.print("Fail.");
}
```

## 4. Nested Loops

```leon
require("basic");

// Nested for loops
for(var(row) = int:1; row <= 3; row = row + 1) {
    for(var(col) = int:1; col <= 3; col = col + 1) {
        basic.print(string:"Row " + string:row + ", Column " + string:col);
    }
}
```

## 5. Comparison Operators

LeonBasic supports the following comparison operators in conditional statements:

* `>`: Greater than
* `<`: Less than
* `>=`: Greater than or equal to
* `<=`: Less than or equal to
* `==`: Equal to
* `!=`: Not equal to

```leon
require("basic");
var(x) = int:10;
var(y) = int:5;

if(x > y) {
    basic.print("x is greater than y");
}

if(x != y) {
    basic.print("x is not equal to y");
}
```
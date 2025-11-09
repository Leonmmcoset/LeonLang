# Time Processing Library (time)

## 1. Introduction

LeonBasic provides the `time` built-in library for handling time-related operations, including getting timestamps, formatting time, program pausing, and more.

## 2. Import Method

Before using the `time` library, you need to import it:

```leon
require("time");
```

## 3. Core Functions

### (1) timestamp() - Get Current Timestamp

Gets the current system time as a millisecond timestamp.

**Parameters**: None
**Return Value**: Integer type, representing milliseconds since 1970-01-01 00:00:00 UTC

**Example**:
```leon
require("basic");
require("time");

// Get current timestamp
var(ts) = time.timestamp();
basic.print(string:"Current timestamp: " + string:var(ts));
```

### (2) formatTime(timestamp, format) - Format Timestamp

Formats a timestamp into a specified format string.

**Parameters**:
- timestamp: Integer type, the timestamp
- format: String type, the format string

**Return Value**: String type, the formatted time string

**Format Specifications**:
- `yyyy`: Four-digit year
- `MM`: Two-digit month (01-12)
- `dd`: Two-digit day (01-31)
- `HH`: Two-digit hour (00-23)
- `mm`: Two-digit minute (00-59)
- `ss`: Two-digit second (00-59)
- `SSS`: Three-digit millisecond (000-999)

**Example**:
```leon
require("basic");
require("time");

// Get current timestamp
var(ts) = time.timestamp();

// Default format (yyyy-MM-dd HH:mm:ss)
var(default_format) = time.formatTime(var(ts), "");
basic.print(string:"Default format: " + var(default_format));

// Custom format
var(custom_format) = time.formatTime(var(ts), "yyyy/MM/dd HH-mm-ss.SSS");
basic.print(string:"Custom format: " + var(custom_format));
```

### (3) sleep(milliseconds) - Pause Program

Pauses program execution for the specified number of milliseconds.

**Parameters**:
- milliseconds: Integer type, the number of milliseconds to pause

**Return Value**: None

**Example**:
```leon
require("basic");
require("time");

basic.print("Waiting...");

// Pause for 2000 milliseconds (2 seconds)
time.sleep(int:2000);

basic.print("Wait completed!");
```

### (4) diffTime(start_time, end_time) - Calculate Time Difference

Calculates the difference between two timestamps (in milliseconds).

**Parameters**:
- start_time: Integer type, the start timestamp
- end_time: Integer type, the end timestamp

**Return Value**: Integer type, the time difference (in milliseconds)

**Example**:
```leon
require("basic");
require("time");

// Record start time
var(start) = time.timestamp();

// Perform some operations
var(sum) = int:0;
for(var(i) = int:0; i < int:1000000; i = i + int:1) {
    sum = sum + i;
}

// Record end time
var(end) = time.timestamp();

// Calculate execution time
var(duration) = time.diffTime(var(start), var(end));
basic.print(string:"Operation execution time: " + string:var(duration) + " milliseconds");
```

### (5) getDateTime() - Get Current Date and Time

Gets the current system date and time as a string.

**Parameters**: None
**Return Value**: String type, current date and time (format: yyyy-MM-dd HH:mm:ss)

**Example**:
```leon
require("basic");
require("time");

// Get current date and time
var(now) = time.getDateTime();
basic.print(string:"Current time: " + var(now));
```

## 4. Comprehensive Examples

### 4.1 Countdown Timer

```leon
require("basic");
require("time");

// Countdown seconds
var(countdown) = int:10;

basic.print(string:"Countdown " + string:var(countdown) + " seconds started...");

// Countdown loop
while(var(countdown) > int:0) {
    basic.print(string:"Remaining time: " + string:var(countdown) + " seconds");
    time.sleep(int:1000);  // Pause for 1 second
    countdown = countdown - int:1;
}

basic.print("Countdown finished!");
```

### 4.2 Performance Timer

```leon
require("basic");
require("time");

// Test performance of different operations
basic.print("Performance test started...");

// Test string concatenation
var(str_start) = time.timestamp();
var(long_str) = string:"";
for(var(i) = int:0; i < int:10000; i = i + int:1) {
    long_str = long_str + string:"a";
}
var(str_end) = time.timestamp();
var(str_time) = time.diffTime(var(str_start), var(str_end));
basic.print(string:"String concatenation 10000 times took: " + string:var(str_time) + " milliseconds");

// Test mathematical operations
var(math_start) = time.timestamp();
var(result) = int:0;
for(var(i) = int:0; i < int:10000000; i = i + int:1) {
    result = (result + i) % int:100;
}
var(math_end) = time.timestamp();
var(math_time) = time.diffTime(var(math_start), var(math_end));
basic.print(string:"Mathematical operations 10 million times took: " + string:var(math_time) + " milliseconds");
```

## 5. Notes

1. Timestamps are based on system time, and modifying the system time will affect the accuracy of timestamps.
2. The `sleep()` function blocks program execution, and the program will be unresponsive during execution.
3. The format string for time formatting is case-insensitive, but uppercase format is recommended for readability.
4. The timestamp precision is millisecond level, suitable for most application scenarios.
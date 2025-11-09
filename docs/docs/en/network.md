# Network

LeonBasic provides network capabilities through the `request` library, allowing you to make HTTP requests and download files.

## 1. HTTP GET Request

```leon
require("basic");
require("request");

var(response) = request.get("https://example.com");
basic.print(response);
```

## 2. Downloading Files

```leon
require("basic");
require("request");

// Download a file from a URL to a local path
request.download("https://example.com/image.jpg", "local.jpg");
basic.print("File downloaded successfully!");
```

## 3. Checking HTTP Status Codes

```leon
require("basic");
require("request");

var(status) = request.check("https://example.com");
basic.print(string:"HTTP Status Code: " + status);
```

## 4. Getting HTTP Headers

```leon
require("basic");
require("request");

var(headers) = request.header("https://example.com");
basic.print(headers);
```

## 5. Getting HTTP Footer Information

```leon
require("basic");
require("request");

var(footer) = request.footer("https://example.com");
basic.print(footer);
```

## 6. Error Handling in Network Operations

It's important to handle errors that may occur during network operations:

```leon
require("basic");
require("request");

try {
    var(response) = request.get("https://nonexistentwebsite123456789.com");
    basic.print(response);
} catch(error) {
    basic.print(string:"Network error: " + error);
}
```

## 7. Working with API Responses

```leon
require("basic");
require("request");

// Fetch data from an API
try {
    var(api_response) = request.get("https://api.example.com/data");
    basic.print(string:"API Response: " + api_response);
} catch(error) {
    basic.print(string:"Failed to fetch API data: " + error);
}
```
# 网络功能

## 1. HTTP GET 请求
```leon
require("request");
var(response) = request.get("https://example.com");
basic.print(response);
```

## 2. 下载文件
```leon
require("request");
request.download("https://example.com/image.jpg", "local.jpg");
```
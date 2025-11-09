use super::{Env, Value};
use std::fs::File;
use std::io::Write;

pub fn register_request_functions(env: &mut Env) {
    // HTTP GET 请求
    env.functions.insert("request.get".to_string(), Box::new(|args| {
        if let Some(Value::String(url)) = args.get(0) {
            let client = reqwest::blocking::Client::new();
            let response = client.get(url).send()
                .map_err(|e| e.to_string())?;
            let text = response.text()
                .map_err(|e| e.to_string())?;
            Ok(Value::String(text))
        } else {
            Err("get函数需要一个字符串参数".to_string())
        }
    }));
    
    // 下载文件
    env.functions.insert("request.download".to_string(), Box::new(|args| {
        if let (Some(Value::String(url)), Some(Value::String(filename))) = 
            (args.get(0), args.get(1)) {
            
            let client = reqwest::blocking::Client::new();
            let response = client.get(url).send()
                .map_err(|e| e.to_string())?;
            
            let mut file = File::create(filename)
                .map_err(|e| e.to_string())?;
            
            let content = response.bytes()
                .map_err(|e| e.to_string())?;
            
            file.write_all(&content)
                .map_err(|e| e.to_string())?;
            
            Ok(Value::Null)
        } else {
            Err("download函数需要两个字符串参数：URL和文件名".to_string())
        }
    }));
    
    // 查看状态码
    env.functions.insert("request.check".to_string(), Box::new(|args| {
        if let Some(Value::String(url)) = args.get(0) {
            let client = reqwest::blocking::Client::new();
            let response = client.get(url).send()
                .map_err(|e| e.to_string())?;
            
            Ok(Value::Int(response.status().as_u16() as i64))
        } else {
            Err("check函数需要一个字符串参数".to_string())
        }
    }));
    
    // 查看头信息
    env.functions.insert("request.header".to_string(), Box::new(|args| {
        if let Some(Value::String(url)) = args.get(0) {
            let client = reqwest::blocking::Client::new();
            let response = client.get(url).send()
                .map_err(|e| e.to_string())?;
            
            let headers = response.headers()
                .iter()
                .map(|(name, value)| format!("{}: {}", name, value.to_str().unwrap_or("无法解析")))
                .collect::<Vec<_>>()
                .join("\n");
            
            Ok(Value::String(headers))
        } else {
            Err("header函数需要一个字符串参数".to_string())
        }
    }));
    
    // 查看尾信息
    env.functions.insert("request.footer".to_string(), Box::new(|args| {
        if let Some(Value::String(url)) = args.get(0) {
            let client = reqwest::blocking::Client::new();
            let response = client.get(url).send()
                .map_err(|e| e.to_string())?;
            
            // 这里简化处理，返回一些基本的响应信息作为"footer"
            let footer = format!("Status: {}\nVersion: {:?}\nURL: {}", 
                response.status(),
                response.version(),
                response.url());
            
            Ok(Value::String(footer))
        } else {
            Err("footer函数需要一个字符串参数".to_string())
        }
    }));
}
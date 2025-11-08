use super::{Env, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

type Function = Box<dyn Fn(Vec<Value>) -> Result<Value, String>>;

pub fn register_basic_functions(env: &mut Env) {
    // 基本输出函数
    env.functions.insert("basic.print".to_string(), Box::new(|args| {
        if args.is_empty() {
            println!();
            return Ok(Value::Null);
        }
        
        let output = args.iter().map(|arg| format_value(arg)).collect::<Vec<String>>().join("");
        println!("{}", output);
        Ok(Value::Null)
    }));
    
    // 执行系统命令
    env.functions.insert("basic.runoscommand".to_string(), Box::new(|args| {
        if let Some(Value::String(cmd)) = args.get(0) {
            let output = Command::new("cmd")
                .args(["/c", cmd])
                .output()
                .map_err(|e| e.to_string())?;
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(Value::String(result))
        } else {
            Err("runoscommand函数需要一个字符串参数".to_string())
        }
    }));
    
    // 设置require路径
    env.functions.insert("basic.setrequirepath".to_string(), Box::new(move |args| {
        if let Some(Value::String(path)) = args.get(0) {
            // 这里简化处理，实际环境中需要更复杂的路径管理
            Ok(Value::String(path.clone()))
        } else {
            Err("setrequirepath函数需要一个字符串参数".to_string())
        }
    }));
    
    // 用户输入
    env.functions.insert("basic.input".to_string(), Box::new(|args| {
        let prompt = if let Some(Value::String(p)) = args.get(0) {
            p.clone()
        } else {
            "".to_string()
        };
        
        print!("{}", prompt);
        std::io::stdout().flush().unwrap_or(());
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .map_err(|e| e.to_string())?;
        
        Ok(Value::String(input.trim_end().to_string()))
    }));
    
    // 暂停函数
    env.functions.insert("basic.pause".to_string(), Box::new(|_| {
        println!("按任意键继续...");
        let _ = std::io::stdin().read(&mut [0u8]).unwrap_or(0);
        Ok(Value::Null)
    }));
    
    // 文件操作 - 打开
    env.functions.insert("basic.open".to_string(), Box::new(|args| {
        if let (Some(Value::String(filename)), Some(Value::String(mode))) = 
            (args.get(0), args.get(1)) {
            
            let file = match mode.as_str() {
                "read" => File::open(filename),
                "write" => File::create(filename),
                "append" => File::options().append(true).create(true).open(filename),
                _ => return Err("不支持的文件模式，使用 read、write 或 append".to_string()),
            };
            
            match file {
                Ok(file) => Ok(Value::File(file)),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("open函数需要两个字符串参数：文件名和模式".to_string())
        }
    }));
    
    // 文件操作 - 写入
    env.functions.insert("basic.write".to_string(), Box::new(|args| {
        if let (Some(Value::File(file)), Some(content)) = 
            (args.get(0), args.get(1)) {
            
            let content_str = format_value(content);
            let mut file_clone = unsafe { (&*file as *const File as *mut File).as_mut().unwrap() };
            file_clone.write_all(content_str.as_bytes())
                .map_err(|e| e.to_string())?;
            
            Ok(Value::Null)
        } else {
            Err("write函数需要一个文件参数和一个内容参数".to_string())
        }
    }));
    
    // 文件操作 - 读取
    env.functions.insert("basic.read".to_string(), Box::new(|args| {
        if let Some(Value::File(file)) = args.get(0) {
            let mut file_clone = unsafe { (&*file as *const File as *mut File).as_mut().unwrap() };
            let mut content = String::new();
            file_clone.read_to_string(&mut content)
                .map_err(|e| e.to_string())?;
            
            Ok(Value::String(content))
        } else {
            Err("read函数需要一个文件参数".to_string())
        }
    }));
    
    // 文件操作 - 关闭
    env.functions.insert("basic.close".to_string(), Box::new(|args| {
        if let Some(Value::File(file)) = args.get(0) {
            // 文件会在超出作用域时自动关闭，这里只是为了API一致性
            Ok(Value::Null)
        } else {
            Err("close函数需要一个文件参数".to_string())
        }
    }));
}

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
}

fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Null => "null".to_string(),
        Value::File(_) => "[文件对象]".to_string(),
    }
}
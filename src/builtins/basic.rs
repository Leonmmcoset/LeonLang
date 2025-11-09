use super::{Env, Value};
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Null => "null".to_string(),
        Value::File(_) => "[文件对象]".to_string(),
    }
}

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
            // 在Windows上，将整个命令作为一个字符串传递给cmd /c
            let output = Command::new("cmd")
                .args(["/c", &cmd])
                .output()
                .map_err(|e| format!("执行命令失败: {}", e.to_string()))?;
            
            // 获取标准输出
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
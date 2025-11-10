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
        Value::File(_) => "[File object]".to_string(),
    }
}

pub fn register_basic_functions(env: &mut Env) {
    // Basic output function
    env.functions.insert("basic.print".to_string(), Box::new(|args| {
        if args.is_empty() {
            println!();
            return Ok(Value::Null);
        }
        
        let output = args.iter().map(|arg| format_value(arg)).collect::<Vec<String>>().join("");
        println!("{}", output);
        Ok(Value::Null)
    }));
    
    // Execute system command
    env.functions.insert("basic.runoscommand".to_string(), Box::new(|args| {
        if let Some(Value::String(cmd)) = args.get(0) {
            // On Windows, pass the entire command as a string to cmd /c
            let output = Command::new("cmd")
                .args(["/c", &cmd])
                .output()
                .map_err(|e| format!("Command execution failed: {}", e.to_string()))?;
            
            // Get standard output
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(Value::String(result))
        } else {
            Err("runoscommand function requires a string parameter".to_string())
        }
    }));
    
    // Set require path
    env.functions.insert("basic.setrequirepath".to_string(), Box::new(move |args| {
        if let Some(Value::String(path)) = args.get(0) {
            // Simplified handling here, real environment needs more complex path management
            Ok(Value::String(path.clone()))
        } else {
            Err("setrequirepath function requires a string parameter".to_string())
        }
    }));
    
    // User input
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
    
    // Pause function
    env.functions.insert("basic.pause".to_string(), Box::new(|_| {
        println!("Press any key to continue...");
        let _ = std::io::stdin().read(&mut [0u8]).unwrap_or(0);
        Ok(Value::Null)
    }));
    
    // File operations - Open
    env.functions.insert("basic.open".to_string(), Box::new(|args| {
        if let (Some(Value::String(filename)), Some(Value::String(mode))) = 
            (args.get(0), args.get(1)) {
            
            let file = match mode.as_str() {
                "read" => File::open(filename),
                "write" => File::create(filename),
                "append" => File::options().append(true).create(true).open(filename),
                _ => return Err("Unsupported file mode, use read, write or append".to_string()),
            };
            
            match file {
                Ok(file) => Ok(Value::File(file)),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("open function requires two string parameters: filename and mode".to_string())
        }
    }));
    
    // File operations - Write
    env.functions.insert("basic.write".to_string(), Box::new(|args| {
        if let (Some(Value::File(file)), Some(content)) = 
            (args.get(0), args.get(1)) {
            
            let content_str = format_value(content);
            let mut file_clone = unsafe { (&*file as *const File as *mut File).as_mut().unwrap() };
            file_clone.write_all(content_str.as_bytes())
                .map_err(|e| e.to_string())?;
            
            Ok(Value::Null)
        } else {
            Err("write function requires a file parameter and a content parameter".to_string())
        }
    }));
    
    // File operations - Read
    env.functions.insert("basic.read".to_string(), Box::new(|args| {
        if let Some(Value::File(file)) = args.get(0) {
            let mut file_clone = unsafe { (&*file as *const File as *mut File).as_mut().unwrap() };
            let mut content = String::new();
            file_clone.read_to_string(&mut content)
                .map_err(|e| e.to_string())?;
            
            Ok(Value::String(content))
        } else {
            Err("read function requires a file parameter".to_string())
        }
    }));
    
    // File operations - Close
    env.functions.insert("basic.close".to_string(), Box::new(|args| {
        if let Some(Value::File(file)) = args.get(0) {
            // File will be automatically closed when out of scope, this is just for API consistency
            Ok(Value::Null)
        } else {
            Err("close function requires a file parameter".to_string())
        }
    }));
}
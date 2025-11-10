use super::{Env, Value};
use std::fs::File;
use std::io::Write;

pub fn register_request_functions(env: &mut Env) {
    // HTTP GET request
    env.functions.insert("request.get".to_string(), Box::new(|args| {
        if let Some(Value::String(url)) = args.get(0) {
            let client = reqwest::blocking::Client::new();
            let response = client.get(url).send()
                .map_err(|e| e.to_string())?;
            let text = response.text()
                .map_err(|e| e.to_string())?;
            Ok(Value::String(text))
        } else {
            Err("get function requires a string parameter".to_string())
        }
    }));
    
    // Download file
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
            Err("download function requires two string parameters: URL and filename".to_string())
        }
    }));
    
    // Check status code
    env.functions.insert("request.check".to_string(), Box::new(|args| {
        if let Some(Value::String(url)) = args.get(0) {
            let client = reqwest::blocking::Client::new();
            let response = client.get(url).send()
                .map_err(|e| e.to_string())?;
            
            Ok(Value::Int(response.status().as_u16() as i64))
        } else {
            Err("check function requires a string parameter".to_string())
        }
    }));
    
    // View header information
    env.functions.insert("request.header".to_string(), Box::new(|args| {
        if let Some(Value::String(url)) = args.get(0) {
            let client = reqwest::blocking::Client::new();
            let response = client.get(url).send()
                .map_err(|e| e.to_string())?;
            
            let headers = response.headers()
                .iter()
                .map(|(name, value)| format!("{}: {}", name, value.to_str().unwrap_or("unparseable")))
                .collect::<Vec<_>>()
                .join("\n");
            
            Ok(Value::String(headers))
        } else {
            Err("header function requires a string parameter".to_string())
        }
    }));
    
    // View footer information
    env.functions.insert("request.footer".to_string(), Box::new(|args| {
        if let Some(Value::String(url)) = args.get(0) {
            let client = reqwest::blocking::Client::new();
            let response = client.get(url).send()
                .map_err(|e| e.to_string())?;
            
            // Simplified handling here, return some basic response information as "footer"
            let footer = format!("Status: {}\nVersion: {:?}\nURL: {}", 
                response.status(),
                response.version(),
                response.url());
            
            Ok(Value::String(footer))
        } else {
            Err("footer function requires a string parameter".to_string())
        }
    }));
}
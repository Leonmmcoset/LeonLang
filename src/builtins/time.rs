use super::{Env, Value};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn register_time_functions(env: &mut Env) {
    // Get current timestamp (milliseconds)
    env.functions.insert("time.timestamp".to_string(), Box::new(|_| {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).map_err(|e| e.to_string())?;
        let timestamp_ms = duration.as_millis() as i64;
        Ok(Value::Int(timestamp_ms))
    }));
    
    // Get current formatted time
    env.functions.insert("time.getDateTime".to_string(), Box::new(|args| {
        let format = if let Some(Value::String(f)) = args.get(0) {
            f.clone()
        } else {
            "%Y-%m-%d %H:%M:%S".to_string() // 默认格式
        };
        
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).map_err(|e| e.to_string())?;
        let timestamp_sec = duration.as_secs();
        
        // 简化的时间格式化（实际项目中可以使用chrono库获得更强大的格式化功能）
        let formatted = format_time_with_pattern(timestamp_sec, &format)?;
        Ok(Value::String(formatted))
    }));
    
    // Format specified timestamp
    env.functions.insert("time.formatTime".to_string(), Box::new(|args| {
        if let (Some(Value::Int(timestamp)), Some(Value::String(format))) = 
            (args.get(0), args.get(1)) {
            
            // 处理毫秒时间戳，转换为秒
            let timestamp_sec = (*timestamp as u64) / 1000;
            let formatted = format_time_with_pattern(timestamp_sec, format)?;
            Ok(Value::String(formatted))
        } else {
            Err("formatTime function requires two parameters: timestamp and format string".to_string())
        }
    }));
    
    // Pause execution for specified milliseconds
    env.functions.insert("time.sleep".to_string(), Box::new(|args| {
        if let Some(Value::Int(ms)) = args.get(0) {
            if *ms < 0 {
                return Err("Sleep time cannot be negative".to_string());
            }
            sleep(Duration::from_millis(*ms as u64));
            Ok(Value::Null)
        } else {
            Err("sleep function requires an integer parameter (milliseconds)".to_string())
        }
    }));
    
    // Calculate difference between two timestamps (milliseconds)
    env.functions.insert("time.diffTime".to_string(), Box::new(|args| {
        if let (Some(Value::Int(start)), Some(Value::Int(end))) = 
            (args.get(0), args.get(1)) {
            
            let diff = *end - *start;
            Ok(Value::Int(diff))
        } else {
            Err("diffTime function requires two integer parameters (start and end timestamps)".to_string())
        }
    }));
    

}

// Simplified time formatting function
// Supported formats: %Y(year), %m(month), %d(day), %H(hour), %M(minute), %S(second)
fn format_time_with_pattern(timestamp: u64, format: &str) -> Result<String, String> {
    // Using simplified time conversion logic here
    // In a real project, it's recommended to use the chrono library for more complex time formatting
    let duration = Duration::from_secs(timestamp);
    let days = duration.as_secs() / (24 * 60 * 60);
    let hours = (duration.as_secs() % (24 * 60 * 60)) / (60 * 60);
    let minutes = (duration.as_secs() % (60 * 60)) / 60;
    let seconds = duration.as_secs() % 60;
    
    // Simplified date calculation (starting from 1970-01-01)
    let year = 1970 + (days / 365);
    let month = ((days % 365) / 30) + 1;
    let day = (days % 30) + 1;
    
    let result = format
        .replace("%Y", &format!("{:04}", year))
        .replace("%m", &format!("{:02}", month))
        .replace("%d", &format!("{:02}", day))
        .replace("%H", &format!("{:02}", hours))
        .replace("%M", &format!("{:02}", minutes))
        .replace("%S", &format!("{:02}", seconds));
    
    Ok(result)
}
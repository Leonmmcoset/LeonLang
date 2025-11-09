use super::{Env, Value};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn register_time_functions(env: &mut Env) {
    // 获取当前时间戳（毫秒）
    env.functions.insert("time.timestamp".to_string(), Box::new(|_| {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).map_err(|e| e.to_string())?;
        let timestamp_ms = duration.as_millis() as i64;
        Ok(Value::Int(timestamp_ms))
    }));
    
    // 获取当前格式化时间
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
    
    // 格式化指定时间戳
    env.functions.insert("time.formatTime".to_string(), Box::new(|args| {
        if let (Some(Value::Int(timestamp)), Some(Value::String(format))) = 
            (args.get(0), args.get(1)) {
            
            // 处理毫秒时间戳，转换为秒
            let timestamp_sec = (*timestamp as u64) / 1000;
            let formatted = format_time_with_pattern(timestamp_sec, format)?;
            Ok(Value::String(formatted))
        } else {
            Err("formatTime函数需要两个参数：时间戳和格式字符串".to_string())
        }
    }));
    
    // 暂停执行指定毫秒数
    env.functions.insert("time.sleep".to_string(), Box::new(|args| {
        if let Some(Value::Int(ms)) = args.get(0) {
            if *ms < 0 {
                return Err("睡眠时间不能为负数".to_string());
            }
            sleep(Duration::from_millis(*ms as u64));
            Ok(Value::Null)
        } else {
            Err("sleep函数需要一个整数参数（毫秒数）".to_string())
        }
    }));
    
    // 计算两个时间戳的差值（毫秒）
    env.functions.insert("time.diffTime".to_string(), Box::new(|args| {
        if let (Some(Value::Int(start)), Some(Value::Int(end))) = 
            (args.get(0), args.get(1)) {
            
            let diff = *end - *start;
            Ok(Value::Int(diff))
        } else {
            Err("diffTime函数需要两个整数参数（开始和结束时间戳）".to_string())
        }
    }));
    
    // 获取当前日期（YYYY-MM-DD格式）
    env.functions.insert("time.today".to_string(), Box::new(|_| {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).map_err(|e| e.to_string())?;
        let timestamp_sec = duration.as_secs();
        
        let formatted = format_time_with_pattern(timestamp_sec, "%Y-%m-%d")?;
        Ok(Value::String(formatted))
    }));
    
    // 获取当前时间（HH:MM:SS格式）
    env.functions.insert("time.currentTimeOnly".to_string(), Box::new(|_| {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).map_err(|e| e.to_string())?;
        let timestamp_sec = duration.as_secs();
        
        let formatted = format_time_with_pattern(timestamp_sec, "%H:%M:%S")?;
        Ok(Value::String(formatted))
    }));
}

// 简化的时间格式化函数
// 支持的格式：%Y(年份), %m(月份), %d(日期), %H(小时), %M(分钟), %S(秒)
fn format_time_with_pattern(timestamp: u64, format: &str) -> Result<String, String> {
    // 这里使用简化的时间转换逻辑
    // 在实际项目中，建议使用chrono库来处理更复杂的时间格式化
    let duration = Duration::from_secs(timestamp);
    let days = duration.as_secs() / (24 * 60 * 60);
    let hours = (duration.as_secs() % (24 * 60 * 60)) / (60 * 60);
    let minutes = (duration.as_secs() % (60 * 60)) / 60;
    let seconds = duration.as_secs() % 60;
    
    // 简化的日期计算（从1970-01-01开始）
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
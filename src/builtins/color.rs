use super::{Env, Value};

// ANSI颜色代码
const COLOR_RESET: &str = "\x1b[0m";
const COLOR_BLACK: &str = "\x1b[30m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_YELLOW: &str = "\x1b[33m";
const COLOR_BLUE: &str = "\x1b[34m";
const COLOR_MAGENTA: &str = "\x1b[35m";
const COLOR_CYAN: &str = "\x1b[36m";
const COLOR_WHITE: &str = "\x1b[37m";

// 亮色变体
const COLOR_BRIGHT_BLACK: &str = "\x1b[90m";
const COLOR_BRIGHT_RED: &str = "\x1b[91m";
const COLOR_BRIGHT_GREEN: &str = "\x1b[92m";
const COLOR_BRIGHT_YELLOW: &str = "\x1b[93m";
const COLOR_BRIGHT_BLUE: &str = "\x1b[94m";
const COLOR_BRIGHT_MAGENTA: &str = "\x1b[95m";
const COLOR_BRIGHT_CYAN: &str = "\x1b[96m";
const COLOR_BRIGHT_WHITE: &str = "\x1b[97m";

fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Null => "null".to_string(),
        Value::File(_) => "[File object]".to_string(),
    }
}

// 根据颜色名称获取ANSI颜色代码
fn get_color_code(color_name: &str) -> &str {
    match color_name.to_lowercase().as_str() {
        "black" => COLOR_BLACK,
        "red" => COLOR_RED,
        "green" => COLOR_GREEN,
        "yellow" => COLOR_YELLOW,
        "blue" => COLOR_BLUE,
        "magenta" => COLOR_MAGENTA,
        "cyan" => COLOR_CYAN,
        "white" => COLOR_WHITE,
        "bright_black" | "gray" | "grey" => COLOR_BRIGHT_BLACK,
        "bright_red" => COLOR_BRIGHT_RED,
        "bright_green" => COLOR_BRIGHT_GREEN,
        "bright_yellow" => COLOR_BRIGHT_YELLOW,
        "bright_blue" => COLOR_BRIGHT_BLUE,
        "bright_magenta" => COLOR_BRIGHT_MAGENTA,
        "bright_cyan" => COLOR_BRIGHT_CYAN,
        "bright_white" => COLOR_BRIGHT_WHITE,
        _ => COLOR_RESET, // 默认返回重置颜色
    }
}

pub fn register_color_functions(env: &mut Env) {
    // 带颜色的打印函数
    env.functions.insert("color.print".to_string(), Box::new(|args| {
        if args.len() < 2 {
            return Err("color.print function requires at least two parameters: color and content".to_string());
        }
        
        // 第一个参数是颜色
        let color_code = if let Value::String(color_name) = &args[0] {
            get_color_code(color_name)
        } else {
            return Err("First parameter must be a color name string".to_string());
        };
        
        // 剩余参数是要打印的内容
        let output = args[1..].iter().map(|arg| format_value(arg)).collect::<Vec<String>>().join("");
        
        // 打印带颜色的内容并重置颜色
        println!("{}{}{}", color_code, output, COLOR_RESET);
        
        Ok(Value::Null)
    }));
    
    // 获取带颜色的字符串（不直接打印）
    env.functions.insert("color.get_color_string".to_string(), Box::new(|args| {
        if args.len() < 2 {
            return Err("color.get_color_string function requires at least two parameters: color and content".to_string());
        }
        
        // 第一个参数是颜色
        let color_code = if let Value::String(color_name) = &args[0] {
            get_color_code(color_name)
        } else {
            return Err("First parameter must be a color name string".to_string());
        };
        
        // 剩余参数是要格式化的内容
        let content = args[1..].iter().map(|arg| format_value(arg)).collect::<Vec<String>>().join("");
        
        // 返回带颜色代码的字符串
        Ok(Value::String(format!("{}{}{}", color_code, content, COLOR_RESET)))
    }));
    
    // 打印带颜色和背景色的函数
    env.functions.insert("color.print_bg".to_string(), Box::new(|args| {
        if args.len() < 3 {
            return Err("color.print_bg function requires at least three parameters: text_color, bg_color, and content".to_string());
        }
        
        // 第一个参数是文字颜色
        let text_color = if let Value::String(color_name) = &args[0] {
            get_color_code(color_name)
        } else {
            return Err("First parameter must be a text color name string".to_string());
        };
        
        // 第二个参数是背景颜色（需要转换为背景色代码）
        let bg_color_code = if let Value::String(color_name) = &args[1] {
            let color_code = get_color_code(color_name);
            // 将前景色代码（3x或9x）转换为背景色代码（4x或10x）
            color_code.replace("[3", "[4").replace("[9", "[10")
        } else {
            return Err("Second parameter must be a background color name string".to_string());
        };
        
        // 剩余参数是要打印的内容
        let output = args[2..].iter().map(|arg| format_value(arg)).collect::<Vec<String>>().join("");
        
        // 打印带颜色和背景色的内容并重置
        println!("{}{}{}{}", bg_color_code, text_color, output, COLOR_RESET);
        
        Ok(Value::Null)
    }));
    
    // 列出所有可用的颜色
    env.functions.insert("color.list_colors".to_string(), Box::new(|_| {
        let colors = [
            "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
            "bright_black (gray/grey)", "bright_red", "bright_green", "bright_yellow", 
            "bright_blue", "bright_magenta", "bright_cyan", "bright_white"
        ];
        
        println!("Available colors:");
        for color in colors {
            println!("  {}", color);
        }
        
        Ok(Value::Null)
    }));
}
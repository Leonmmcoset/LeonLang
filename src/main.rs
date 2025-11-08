use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, stdin, Write};
use std::path::Path;

// 定义值的类型
#[derive(Debug)]
enum Value {
    String(String),
    Int(i64),
    Float(f64),
    // 添加File类型用于文件操作
    #[allow(dead_code)]
    File(File),
    // Null 变体暂时未使用，保留以便将来扩展
    #[allow(dead_code)]
    Null,
}

// 函数类型别名
type Function = Box<dyn Fn(Vec<Value>) -> Result<Value, String>>;

// 定义执行环境
struct Env {
    variables: HashMap<String, Value>,
    loaded_packages: HashMap<String, bool>,
    functions: HashMap<String, Function>,
}

impl Env {
    fn new() -> Self {
        let mut env = Self {
            variables: HashMap::new(),
            loaded_packages: HashMap::new(),
            functions: HashMap::new(),
        };
        
        // 注册基本函数
        register_basic_functions(&mut env);
        
        env
    }
    
    fn parse_and_execute(&mut self, code: &str) -> Result<(), String> {
        let lines: Vec<&str> = code.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let trimmed = lines[i].trim();
            if trimmed.is_empty() || trimmed.starts_with("//") {
                i += 1;
                continue;
            }
            
            // 特殊处理if语句（可能包含多行代码块）
            if trimmed.starts_with("if(") {
                let (result, new_index) = self.execute_if_statement(&lines[i..])?;
                if let Err(e) = result {
                    return Err(format!("执行错误: {}", e));
                }
                i += new_index;
            } else {
                // 处理单行语句
                if let Err(e) = self.execute_line(trimmed) {
                    return Err(format!("执行错误: {}", e));
                }
                i += 1;
            }
        }
        
        Ok(())
    }
    
    fn execute_line(&mut self, line: &str) -> Result<(), String> {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with("//") {
            return Ok(());
        }
        
        // 检查是否需要分号
        let needs_semicolon = !trimmed_line.contains("{") && 
                            !trimmed_line.contains("}") && 
                            !trimmed_line.ends_with(';') && 
                            (trimmed_line.contains('.') && trimmed_line.contains('(') || 
                             trimmed_line.starts_with("var(") && trimmed_line.contains(" = ") || 
                             trimmed_line.starts_with("require("));
        
        if needs_semicolon {
            return Err("语法错误: 语句应以分号结尾".to_string());
        }
        
        // 处理 require 语句
        if trimmed_line.starts_with("require(") {
            return self.handle_require(trimmed_line);
        }
        
        // 处理变量定义
        if trimmed_line.starts_with("var(") && trimmed_line.contains(" = ") {
            return self.handle_variable_definition(trimmed_line);
        }
        
        // 处理函数调用
        if trimmed_line.contains('.') && trimmed_line.contains('(') {
            return self.handle_function_call(trimmed_line);
        }
        
        Ok(())
    }
    
    // 处理if条件语句
    fn handle_if_condition(&mut self, _line: &str) -> Result<(), String> {
        // 这个方法现在只是为了保持接口兼容，实际功能在execute_if_statement中实现
        Ok(())
    }
    
    // 执行if语句（包含多行代码块）
    fn execute_if_statement(&mut self, lines: &[&str]) -> Result<(Result<(), String>, usize), String> {
        let first_line = lines[0].trim();
        
        // 提取条件部分
        let cond_start = first_line.find('(').ok_or("if语句格式错误")? + 1;
        let cond_end = first_line.find(')').ok_or("if语句格式错误")?;
        let condition_str = &first_line[cond_start..cond_end];
        
        // 评估条件
        let condition_result = self.evaluate_condition(condition_str)?;
        
        let mut line_index = 1;
        
        // 处理if代码块
        let mut brace_count = 0;
        let mut in_if_block = false;
        
        // 查找并处理if代码块
        while line_index < lines.len() {
            let line = lines[line_index];
            let trimmed = line.trim();
            
            // 检查是否是else语句
            if trimmed == "else" {
                // 确保已经处理完if代码块
                if !in_if_block || brace_count > 0 {
                    // 如果还在if代码块内部就遇到了else，这是语法错误
                    return Err("if语句格式错误: 遇到未闭合的if代码块".to_string());
                }
                break;
            }
            
            // 如果还没找到代码块开始，继续查找
            if !in_if_block {
                // 检查行中是否包含大括号
                for (i, c) in line.chars().enumerate() {
                    if c == '{' {
                        brace_count += 1;
                        in_if_block = true;
                        // 如果大括号后还有内容，需要处理
                        let after_brace = &line[i+1..].trim();
                        if !after_brace.is_empty() && !after_brace.starts_with("//") && condition_result {
                            // 为代码块内的语句添加分号（如果需要）
                            let line_to_execute = if !after_brace.contains("{") && !after_brace.contains("}") && 
                                                !after_brace.ends_with(';') && 
                                                (after_brace.contains('.') && after_brace.contains('(') || 
                                                 after_brace.starts_with("var(") && after_brace.contains(" = ")) {
                                after_brace.to_string() + ";"
                            } else {
                                after_brace.to_string()
                            };
                            
                            if let Err(e) = self.execute_line(&line_to_execute) {
                                return Ok((Err(e), line_index + 1));
                            }
                        }
                        break;
                    }
                }
                line_index += 1;
                continue;
            }
            
            // 处理代码块内容
            if in_if_block {
                // 统计大括号
                for c in line.chars() {
                    if c == '{' {
                        brace_count += 1;
                    } else if c == '}' {
                        brace_count -= 1;
                        // 代码块结束
                        if brace_count == 0 {
                            in_if_block = false;
                            line_index += 1;
                            break;
                        }
                    }
                }
                
                // 如果代码块还没结束，处理当前行内容
                if in_if_block && condition_result {
                    if !trimmed.is_empty() && !trimmed.starts_with("//") {
                        // 为代码块内的语句添加分号（如果需要）
                        let line_to_execute = if !trimmed.contains("{") && !trimmed.contains("}") && 
                                            !trimmed.ends_with(';') && 
                                            (trimmed.contains('.') && trimmed.contains('(') || 
                                             trimmed.starts_with("var(") && trimmed.contains(" = ")) {
                            trimmed.to_string() + ";"
                        } else {
                            trimmed.to_string()
                        };
                        
                        if let Err(e) = self.execute_line(&line_to_execute) {
                            return Ok((Err(e), line_index + 1));
                        }
                    }
                }
                line_index += 1;
            }
        }
        
        // 如果条件为true，直接返回，不执行else部分
        if condition_result {
            return Ok((Ok(()), line_index));
        }
        
        // 处理else部分（只有当条件为false时才执行）
        if line_index < lines.len() {
            let trimmed = lines[line_index].trim();
            if trimmed == "else" {
                line_index += 1;
                
                // 查找并处理else代码块
                brace_count = 0;
                let mut in_else_block = false;
                
                while line_index < lines.len() {
                    let line = lines[line_index];
                    let trimmed_else = line.trim();
                    
                    // 如果还没找到代码块开始，继续查找
                    if !in_else_block {
                        // 检查行中是否包含大括号
                        for (i, c) in line.chars().enumerate() {
                            if c == '{' {
                                brace_count += 1;
                                in_else_block = true;
                                // 如果大括号后还有内容，需要处理
                                let after_brace = &line[i+1..].trim();
                                if !after_brace.is_empty() && !after_brace.starts_with("//") {
                                    // 为代码块内的语句添加分号（如果需要）
                                    let line_to_execute = if !after_brace.contains("{") && !after_brace.contains("}") && 
                                                        !after_brace.ends_with(';') && 
                                                        (after_brace.contains('.') && after_brace.contains('(') || 
                                                         after_brace.starts_with("var(") && after_brace.contains(" = ")) {
                                        after_brace.to_string() + ";"
                                    } else {
                                        after_brace.to_string()
                                    };
                                    
                                    if let Err(e) = self.execute_line(&line_to_execute) {
                                        return Ok((Err(e), line_index + 1));
                                    }
                                }
                                break;
                            }
                        }
                        line_index += 1;
                        continue;
                    }
                    
                    // 处理代码块内容
                    if in_else_block {
                        // 统计大括号
                        for c in line.chars() {
                            if c == '{' {
                                brace_count += 1;
                            } else if c == '}' {
                                brace_count -= 1;
                                // 代码块结束
                                if brace_count == 0 {
                                    in_else_block = false;
                                    line_index += 1;
                                    break;
                                }
                            }
                        }
                        
                        // 如果代码块还没结束，处理当前行内容
                        if in_else_block {
                            if !trimmed_else.is_empty() && !trimmed_else.starts_with("//") {
                                // 为代码块内的语句添加分号（如果需要）
                                let line_to_execute = if !trimmed_else.contains("{") && !trimmed_else.contains("}") && 
                                                    !trimmed_else.ends_with(';') && 
                                                    (trimmed_else.contains('.') && trimmed_else.contains('(') || 
                                                     trimmed_else.starts_with("var(") && trimmed_else.contains(" = ")) {
                                    trimmed_else.to_string() + ";"
                                } else {
                                    trimmed_else.to_string()
                                };
                                
                                if let Err(e) = self.execute_line(&line_to_execute) {
                                    return Ok((Err(e), line_index + 1));
                                }
                            }
                        }
                        line_index += 1;
                    }
                }
            }
        }
        
        Ok((Ok(()), line_index))
    }
    
    // 评估条件表达式
    fn evaluate_condition(&mut self, condition: &str) -> Result<bool, String> {
        // 简化版本，仅支持数字比较
        let condition = condition.trim();
        println!("DEBUG: 正在评估条件: {}", condition);
        
        // 检查 > 比较
        if let Some(pos) = condition.find('>') {
            let left = &condition[..pos].trim();
            let right = &condition[pos+1..].trim();
            println!("DEBUG: 左侧: {}, 右侧: {}", left, right);
            
            // 解析两边的值，尝试获取变量的值
            let left_val = self.get_value_or_constant(left)?;
            let right_val = self.get_value_or_constant(right)?;
            
            println!("DEBUG: 左侧值类型: {:?}, 右侧值类型: {:?}", 
                     match &left_val {
                         Value::String(_) => "String",
                         Value::Int(_) => "Int",
                         Value::Float(_) => "Float",
                         _ => "Other",
                     },
                     match &right_val {
                         Value::String(_) => "String",
                         Value::Int(_) => "Int",
                         Value::Float(_) => "Float",
                         _ => "Other",
                     });
            
            // 打印具体值
            match (&left_val, &right_val) {
                (Value::Int(l), Value::Int(r)) => println!("DEBUG: 左侧值: {}, 右侧值: {}", l, r),
                (Value::Int(l), Value::Float(r)) => println!("DEBUG: 左侧值: {}, 右侧值: {}", l, r),
                (Value::Float(l), Value::Int(r)) => println!("DEBUG: 左侧值: {}, 右侧值: {}", l, r),
                (Value::Float(l), Value::Float(r)) => println!("DEBUG: 左侧值: {}, 右侧值: {}", l, r),
                _ => (),
            }
            
            // 比较
            let result = match (left_val, right_val) {
                (Value::Int(left_num), Value::Int(right_num)) => left_num > right_num,
                (Value::Int(left_num), Value::Float(right_num)) => (left_num as f64) > right_num,
                (Value::Float(left_num), Value::Int(right_num)) => left_num > (right_num as f64),
                (Value::Float(left_num), Value::Float(right_num)) => left_num > right_num,
                _ => return Err("比较操作只能用于数字类型".to_string()),
            };
            println!("DEBUG: 比较结果: {}", result);
            Ok(result)
        } 
        // 检查 < 比较
        else if let Some(pos) = condition.find('<') {
            let left = &condition[..pos].trim();
            let right = &condition[pos+1..].trim();
            
            // 解析两边的值，尝试获取变量的值
            let left_val = self.get_value_or_constant(left)?;
            let right_val = self.get_value_or_constant(right)?;
            
            // 比较
            match (left_val, right_val) {
                (Value::Int(left_num), Value::Int(right_num)) => Ok(left_num < right_num),
                (Value::Int(left_num), Value::Float(right_num)) => Ok((left_num as f64) < right_num),
                (Value::Float(left_num), Value::Int(right_num)) => Ok(left_num < (right_num as f64)),
                (Value::Float(left_num), Value::Float(right_num)) => Ok(left_num < right_num),
                _ => Err("比较操作只能用于数字类型".to_string()),
            }
        }
        // 检查 == 比较
        else if let Some(pos) = condition.find("==") {
            let left = &condition[..pos].trim();
            let right = &condition[pos+2..].trim();
            
            // 解析两边的值，尝试获取变量的值
            let left_val = self.get_value_or_constant(left)?;
            let right_val = self.get_value_or_constant(right)?;
            
            // 比较
            match (left_val, right_val) {
                (Value::String(s1), Value::String(s2)) => Ok(s1 == s2),
                (Value::Int(i1), Value::Int(i2)) => Ok(i1 == i2),
                (Value::Float(f1), Value::Float(f2)) => Ok(f1 == f2),
                (Value::Int(i), Value::Float(f)) => Ok((i as f64) == f),
                (Value::Float(f), Value::Int(i)) => Ok(f == (i as f64)),
                (Value::Null, Value::Null) => Ok(true),
                _ => Ok(false), // 不同类型的值不相等
            }
        }
        // 检查 != 比较
        else if let Some(pos) = condition.find("!=") {
            let left = &condition[..pos].trim();
            let right = &condition[pos+2..].trim();
            
            // 解析两边的值，尝试获取变量的值
            let left_val = self.get_value_or_constant(left)?;
            let right_val = self.get_value_or_constant(right)?;
            
            // 比较
            match (left_val, right_val) {
                (Value::String(s1), Value::String(s2)) => Ok(s1 != s2),
                (Value::Int(i1), Value::Int(i2)) => Ok(i1 != i2),
                (Value::Float(f1), Value::Float(f2)) => Ok(f1 != f2),
                (Value::Int(i), Value::Float(f)) => Ok((i as f64) != f),
                (Value::Float(f), Value::Int(i)) => Ok(f != (i as f64)),
                (Value::Null, Value::Null) => Ok(false),
                _ => Ok(true), // 不同类型的值不相等
            }
        }
        else {
            Err("不支持的条件操作符".to_string())
        }
    }
    
    // 获取值或常量，支持变量引用
    fn get_value_or_constant(&mut self, value_str: &str) -> Result<Value, String> {
        // 如果是变量引用（如 "var(a)"）
        if value_str.starts_with("var(") && value_str.ends_with(")") {
            let var_name = &value_str[4..value_str.len()-1];
            if let Some(val) = self.variables.get(var_name) {
                // 根据值的类型创建新的值
                return Ok(match val {
                    Value::String(s) => Value::String(s.clone()),
                    Value::Int(i) => Value::Int(*i),
                    Value::Float(f) => Value::Float(*f),
                    Value::Null => Value::Null,
                    _ => return Err("不支持的变量类型用于条件比较".to_string()),
                });
            } else {
                return Err(format!("未定义的变量: {}", var_name));
            }
        }
        
        // 如果是直接引用变量名（如 "a"）
        if !value_str.contains(":") && !value_str.contains(".") && !value_str.contains("(") {
            if let Some(val) = self.variables.get(value_str) {
                // 根据值的类型创建新的值
                return Ok(match val {
                    Value::String(s) => Value::String(s.clone()),
                    Value::Int(i) => Value::Int(*i),
                    Value::Float(f) => Value::Float(*f),
                    Value::Null => Value::Null,
                    _ => return Err("不支持的变量类型用于条件比较".to_string()),
                });
            }
        }
        
        // 否则尝试解析为常量值
        self.parse_value_without_expression(value_str)
    }
    
    // 解析值或常量数字
    fn parse_value_or_constant(&self, value_str: &str) -> Result<Value, String> {
        // 检查是否是变量引用（var(a)格式）
        if value_str.starts_with("var(") {
            let var_name = value_str.trim_start_matches("var(").trim_end_matches(")");
            if let Some(val) = self.variables.get(var_name) {
                // 手动复制值，避免使用clone
                return match val {
                    Value::String(s) => Ok(Value::String(s.clone())),
                    Value::Int(i) => Ok(Value::Int(*i)),
                    Value::Float(f) => Ok(Value::Float(*f)),
                    // 对于File类型，由于没有实现Clone，这里简化处理
                    Value::File(_) => Err("无法比较文件类型".to_string()),
                    Value::Null => Ok(Value::Null),
                };
            }
        }
        
        // 检查是否是简单变量名（如a、b等）
        if !value_str.contains(":") && !value_str.contains(".") && !value_str.contains("(") && !value_str.contains(")") && !value_str.contains("\"") {
            if let Some(val) = self.variables.get(value_str) {
                // 手动复制值，避免使用clone
                match val {
                    Value::String(s) => return Ok(Value::String(s.clone())),
                    Value::Int(i) => return Ok(Value::Int(*i)),
                    Value::Float(f) => return Ok(Value::Float(*f)),
                    // 对于File类型，由于没有实现Clone，这里简化处理
                    Value::File(_) => return Err("无法比较文件类型".to_string()),
                    Value::Null => return Ok(Value::Null),
                };
            }
        }
        
        // 尝试解析为整数常量
        if let Ok(num) = value_str.parse::<i64>() {
            return Ok(Value::Int(num));
        }
        
        // 尝试解析为普通值
        self.parse_value(value_str)
    }
    
    // 比较两个值
    fn compare_values(&self, left: &Value, right: &Value, operator: &str) -> Result<bool, String> {
        // 简化实现 - 目前只处理整数比较
        if let (Value::Int(left_int), Value::Int(right_int)) = (left, right) {
            match operator {
                ">" => Ok(left_int > right_int),
                "<" => Ok(left_int < right_int),
                ">=" => Ok(left_int >= right_int),
                "<=" => Ok(left_int <= right_int),
                "==" => Ok(left_int == right_int),
                _ => Err("不支持的比较操作符".to_string()),
            }
        } else {
            Err("目前只支持整数比较".to_string())
        }
    }
    
    fn handle_require(&mut self, line: &str) -> Result<(), String> {
        // 去除末尾的分号
        let line_without_semicolon = line.trim_end_matches(';');
        
        // 简单解析 require 语句
        let start = line_without_semicolon.find('"').ok_or("require语句格式错误")? + 1;
        let end = line_without_semicolon.rfind('"').ok_or("require语句格式错误")?;
        let lib_name = &line_without_semicolon[start..end];
        
        // 标记包已加载
        self.loaded_packages.insert(lib_name.to_string(), true);
        Ok(())
    }
    

    
    fn handle_variable_definition(&mut self, line: &str) -> Result<(), String> {
        let parts: Vec<&str> = line.split(" = ").collect();
        if parts.len() != 2 {
            return Err("变量定义格式错误".to_string());
        }
        
        let var_name_part = parts[0];
        let var_name = var_name_part.trim_start_matches("var(").trim_end_matches(")");
        
        let value_part = parts[1].trim_end_matches(';');
        
        // 检查是否是函数调用
        let value = if value_part.contains(".") && value_part.contains("(") && value_part.contains(")") {
            // 调用函数并获取返回值
            self.execute_function_call(value_part)?
        } else {
            // 解析普通值
            self.parse_value(value_part)?
        };
        
        self.variables.insert(var_name.to_string(), value);
        Ok(())
    }
    
    fn handle_function_call(&mut self, function_call: &str) -> Result<(), String> {
        // 移除分号
        let func_call = function_call.trim_end_matches(';');
        
        // 执行函数调用并忽略返回值
        self.execute_function_call(func_call)?;
        
        Ok(())
    }
    
    fn execute_function_call(&mut self, function_call: &str) -> Result<Value, String> {
        // 解析函数名和参数
        let func_name_end = function_call.find('(').ok_or("函数调用格式错误")?;
        let func_name = &function_call[..func_name_end];
        
        // 检查函数是否存在
        if let Some(func) = self.functions.get(func_name) {
            // 解析参数
            let args_str = &function_call[func_name_end + 1..function_call.len() - 1];
            let args = if !args_str.trim().is_empty() {
                // 特殊处理basic.input函数
                if func_name == "basic.input" {
                    // 提取引号内的内容
                    if args_str.contains('"') {
                        let start = args_str.find('"').ok_or("参数格式错误")? + 1;
                        let end = args_str.rfind('"').ok_or("参数格式错误")?;
                        let content = &args_str[start..end];
                        vec![Value::String(content.to_string())]
                    } else {
                        vec![Value::String(args_str.trim().to_string())]
                    }
                } else {
                    // 处理其他函数的参数
                    let parts: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();
                    let mut args_vec = Vec::new();
                    
                    for part in parts {
                        let value = self.parse_value(part)?;
                        args_vec.push(value);
                    }
                    args_vec
                }
            } else {
                Vec::new()
            };
            
            // 调用函数
            func(args)
        } else {
            Err(format!("未定义的函数: {}", func_name))
        }
    }
    
    fn parse_value(&self, value_str: &str) -> Result<Value, String> {
        // 先检查是否包含字符串拼接表达式
        if value_str.contains(" + ") {
            return self.evaluate_expression(value_str);
        }
        
        // 处理变量引用
        if value_str.starts_with("var(") {
            let var_name = value_str.trim_start_matches("var(").trim_end_matches(")");
            if let Some(val) = self.variables.get(var_name) {
                return match val {
                    Value::String(s) => Ok(Value::String(s.clone())),
                    Value::Int(i) => Ok(Value::Int(*i)),
                    Value::Float(f) => Ok(Value::Float(*f)),
                    Value::Null => Ok(Value::Null),
                    Value::File(_) => Err("不能直接引用文件句柄".to_string()),
                };
            } else {
                return Err(format!("未定义的变量: {}", var_name));
            }
        }
        
        // 处理字符串
        if value_str.starts_with("string:") {
            let content_part = value_str.trim_start_matches("string:");
            
            // 检查是否是 string:var(变量名) 形式
            if content_part.starts_with("var(") {
                // 递归调用 parse_value_without_expression 来获取变量的值
                let var_value = self.parse_value_without_expression(content_part)?;
                // 将获取到的值转换为字符串
                return match var_value {
                    Value::String(s) => Ok(Value::String(s.clone())),
                    Value::Int(i) => Ok(Value::String(i.to_string())),
                    Value::Float(f) => Ok(Value::String(f.to_string())),
                    Value::Null => Ok(Value::String("null".to_string())),
                    Value::File(_) => Ok(Value::String("[file handle]".to_string())),
                };
            }
            
            // 普通字符串字面量
            let content = content_part.trim_matches('"');
            return Ok(Value::String(content.to_string()));
        }
        
        // 处理整数
        if value_str.starts_with("int:") {
            let num_str = value_str.trim_start_matches("int:");
            if let Ok(num) = num_str.parse::<i64>() {
                return Ok(Value::Int(num));
            }
        }
        
        // 处理浮点数
        if value_str.starts_with("float:") {
            let num_str = value_str.trim_start_matches("float:");
            if let Ok(num) = num_str.parse::<f64>() {
                return Ok(Value::Float(num));
            }
        }
        
        // 默认作为字符串处理
        Ok(Value::String(value_str.to_string()))
    }
    
    fn evaluate_expression(&self, expr: &str) -> Result<Value, String> {
        // 处理字符串拼接表达式
        let parts: Vec<&str> = expr.split(" + ").collect();
        let mut result = String::new();
        
        for part in parts {
            // 递归解析每个部分的值
            let value = self.parse_value_without_expression(part.trim())?;
            
            // 将所有值转换为字符串并拼接
                match value {
                    Value::String(s) => result.push_str(&s),
                    Value::Int(i) => result.push_str(&i.to_string()),
                    Value::Float(f) => result.push_str(&f.to_string()),
                    Value::Null => result.push_str("null"),
                    Value::File(_) => result.push_str("[file handle]"),
                }
        }
        
        Ok(Value::String(result))
    }
    
    // 不带表达式解析的版本，避免递归
    fn parse_value_without_expression(&self, value_str: &str) -> Result<Value, String> {
        // 处理变量引用
        if value_str.starts_with("var(") {
            let var_name = value_str.trim_start_matches("var(").trim_end_matches(")");
            if let Some(val) = self.variables.get(var_name) {
                return match val {
                    Value::String(s) => Ok(Value::String(s.clone())),
                    Value::Int(i) => Ok(Value::Int(*i)),
                    Value::Float(f) => Ok(Value::Float(*f)),
                    Value::Null => Ok(Value::Null),
                    Value::File(_) => Err("不能直接引用文件句柄".to_string()),
                };
            } else {
                return Err(format!("未定义的变量: {}", var_name));
            }
        }
        
        // 处理字符串
        if value_str.starts_with("string:") {
            let content_part = value_str.trim_start_matches("string:");
            
            // 检查是否是 string:var(变量名) 形式
            if content_part.starts_with("var(") {
                // 获取变量名
                let var_name = content_part.trim_start_matches("var(").trim_end_matches(")");
                if let Some(val) = self.variables.get(var_name) {
                    // 将变量的值转换为字符串
                    return match val {
                        Value::String(s) => Ok(Value::String(s.clone())),
                        Value::Int(i) => Ok(Value::String(i.to_string())),
                        Value::Float(f) => Ok(Value::String(f.to_string())),
                        Value::Null => Ok(Value::String("null".to_string())),
                        Value::File(_) => Ok(Value::String("[file handle]".to_string())),
                    };
                } else {
                    return Err(format!("未定义的变量: {}", var_name));
                }
            }
            
            // 普通字符串字面量
            let content = content_part.trim_matches('"');
            return Ok(Value::String(content.to_string()));
        }
        
        // 处理整数
        if value_str.starts_with("int:") {
            let num_str = value_str.trim_start_matches("int:");
            if let Ok(num) = num_str.parse::<i64>() {
                return Ok(Value::Int(num));
            }
        }
        
        // 处理浮点数
        if value_str.starts_with("float:") {
            let num_str = value_str.trim_start_matches("float:");
            if let Ok(num) = num_str.parse::<f64>() {
                return Ok(Value::Float(num));
            }
        }
        
        // 默认作为字符串处理
        Ok(Value::String(value_str.to_string()))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // 检查是否启动shell模式
    if args.len() == 2 && args[1] == "--shell" {
        start_shell();
        return;
    }
    
    // 正常的文件执行模式
    if args.len() < 2 {
        println!("LeonBasic 解释器 v0.1.0");
        println!("用法: leonlang <文件>");
        println!("用法: leonlang --shell  # 启动交互式shell");
        return;
    }
    
    let file_path = &args[1];
    if !Path::new(file_path).exists() {
        eprintln!("文件不存在: {}", file_path);
        return;
    }
    
    let mut env = Env::new();
    
    match File::open(file_path) {
        Ok(mut file) => {
            let mut content = String::new();
            if let Err(e) = file.read_to_string(&mut content) {
                eprintln!("读取文件失败: {}", e);
            } else if let Err(e) = env.parse_and_execute(&content) {
                eprintln!("{}", e);
            }
        },
        Err(e) => eprintln!("打开文件失败: {}", e),
    }
}

// 启动交互式shell
fn start_shell() {
    println!("LeonBasic Shell v0.1.0");
    println!("输入 'exit' 退出shell");
    println!("输入 'help' 查看帮助信息");
    println!("------------------------");
    
    let mut env = Env::new();
    
    // 默认加载basic库
    if let Err(e) = env.handle_require("require(\"basic\")") {
        println!("警告: 无法加载basic库: {}", e);
    }
    
    loop {
        print!("leon> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        let mut input = String::new();
        if let Err(e) = stdin().read_line(&mut input) {
            println!("读取输入失败: {}", e);
            continue;
        }
        
        let line = input.trim();
        
        // 处理特殊命令
        if line == "exit" || line == "quit" || line == "q" {
            println!("再见!");
            break;
        } else if line == "help" || line == "h" {
            println!("可用命令:");
            println!("  exit/quit/q  - 退出shell");
            println!("  help/h       - 显示此帮助信息");
            println!("  clear        - 清屏");
            println!("基本语法示例:");
            println!("  var(a) = string:\"Hello\";  # 定义变量");
            println!("  basic.print(var(a));        # 打印变量");
            println!("  require(\"basic\");          # 加载库");
            continue;
        } else if line == "clear" {
            #[cfg(target_os = "windows")]
            { let _ = std::process::Command::new("cls").status(); }
            #[cfg(not(target_os = "windows"))]
            { let _ = std::process::Command::new("clear").status(); }
            continue;
        }
        
        // 执行LeonBasic代码
        if !line.is_empty() && !line.starts_with("//") {
            if let Err(e) = env.execute_line(line) {
                println!("错误: {}", e);
            }
        }
    }
}

// 格式化值为字符串
fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Null => "null".to_string(),
        Value::File(_) => "[file handle]".to_string(),
    }
}

// 注册基本函数
fn register_basic_functions(env: &mut Env) {
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
    
    // 用户输入
        env.functions.insert("basic.input".to_string(), Box::new(|args| {
            // 获取提示信息参数
            let prompt = if let Some(Value::String(p)) = args.get(0) {
                p.clone()
            } else {
                "请输入: ".to_string()
            };
            
            // 打印提示信息并刷新缓冲区
            print!("{}", prompt);
            std::io::stdout().flush().unwrap_or(());
            
            // 读取用户输入
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_n) => {
                    // 保留原始输入内容（包括换行符），不进行trim处理
                    Ok(Value::String(input))
                },
                Err(_) => Err("读取输入失败".to_string())
            }
        }));
    
    // 暂停函数
    env.functions.insert("basic.pause".to_string(), Box::new(|_| {
        println!("按任意键继续...");
        let _ = std::io::stdin().read(&mut [0u8]).unwrap_or(0);
        Ok(Value::Null)
    }));
}
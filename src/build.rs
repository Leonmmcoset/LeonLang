use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

// 字节码文件魔数
const MAGIC_NUMBER: [u8; 4] = *b"LEON";
// 字节码版本
const BYTECODE_VERSION: u8 = 1;
// 加密密钥
const ENCRYPTION_KEY: &[u8] = b"LEON_BASIC_ENCRYPT";

/// 简单的XOR加密函数
fn encrypt(data: &[u8]) -> Vec<u8> {
    data.iter().enumerate().map(|(i, &byte)| {
        byte ^ ENCRYPTION_KEY[i % ENCRYPTION_KEY.len()]
    }).collect()
}

/// 解密函数（与加密函数相同，因为XOR是对称的）
fn decrypt(data: &[u8]) -> Vec<u8> {
    // XOR加密是对称的，所以解密使用相同的函数
    encrypt(data)
}

/// 将LeonBasic源代码编译为字节码文件
pub fn compile_to_bytecode(source_path: &str, output_path: &str) -> Result<(), String> {
    // 检查源文件是否存在
    if !Path::new(source_path).exists() {
        return Err(format!("Source file not found: {}", source_path));
    }

    // 读取源代码文件
    let mut source_file = File::open(source_path)
        .map_err(|e| format!("Failed to open source file: {}", e))?;
    
    let mut source_code = String::new();
    source_file.read_to_string(&mut source_code)
        .map_err(|e| format!("Failed to read source file: {}", e))?;

    // 创建或截断输出文件
    let mut bytecode_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path)
        .map_err(|e| format!("Failed to create output file: {}", e))?;

    // 写入魔数
    bytecode_file.write_all(&MAGIC_NUMBER)
        .map_err(|e| format!("Failed to write magic number: {}", e))?;
    
    // 写入版本号
    bytecode_file.write_all(&[BYTECODE_VERSION])
        .map_err(|e| format!("Failed to write version: {}", e))?;
    
    // 写入源代码长度（4字节，小端序）
    let source_len = source_code.len() as u32;
    bytecode_file.write_all(&source_len.to_le_bytes())
        .map_err(|e| format!("Failed to write source length: {}", e))?;
    
    // 对源代码进行加密
    let encrypted_code = encrypt(source_code.as_bytes());
    
    // 写入加密后的源代码内容
    bytecode_file.write_all(&encrypted_code)
        .map_err(|e| format!("Failed to write encrypted source code: {}", e))?;

    Ok(())
}

/// 从字节码文件中读取LeonBasic源代码
pub fn read_from_bytecode(bytecode_path: &str) -> Result<String, String> {
    // 检查字节码文件是否存在
    if !Path::new(bytecode_path).exists() {
        return Err(format!("Bytecode file not found: {}", bytecode_path));
    }

    // 读取字节码文件
    let mut bytecode_file = File::open(bytecode_path)
        .map_err(|e| format!("Failed to open bytecode file: {}", e))?;
    
    // 读取并验证魔数
    let mut magic_number = [0u8; 4];
    bytecode_file.read_exact(&mut magic_number)
        .map_err(|e| format!("Failed to read magic number: {}", e))?;
    
    if magic_number != MAGIC_NUMBER {
        return Err("Invalid bytecode file format: wrong magic number".to_string());
    }
    
    // 读取版本号
    let mut version = [0u8; 1];
    bytecode_file.read_exact(&mut version)
        .map_err(|e| format!("Failed to read version: {}", e))?;
    
    // 检查版本兼容性
    if version[0] > BYTECODE_VERSION {
        return Err(format!("Unsupported bytecode version: {}", version[0]));
    }
    
    // 读取源代码长度
    let mut source_len_bytes = [0u8; 4];
    bytecode_file.read_exact(&mut source_len_bytes)
        .map_err(|e| format!("Failed to read source length: {}", e))?;
    
    let source_len = u32::from_le_bytes(source_len_bytes) as usize;
    
    // 读取源代码内容
    let mut source_code = vec![0u8; source_len];
    bytecode_file.read_exact(&mut source_code)
        .map_err(|e| format!("Failed to read source code: {}", e))?;
    
    // 解密源代码
    let decrypted_code = decrypt(&source_code);
    
    // 转换为字符串
    String::from_utf8(decrypted_code)
        .map_err(|e| format!("Failed to decode decrypted source code: {}", e))
}

/// 检查文件是否为有效的LeonBasic字节码文件
pub fn is_bytecode_file(file_path: &str) -> bool {
    // 检查文件扩展名
    if !file_path.ends_with(".lb") {
        return false;
    }
    
    // 尝试打开文件并读取魔数
    if let Ok(mut file) = File::open(file_path) {
        let mut magic_number = [0u8; 4];
        if let Ok(_) = file.read_exact(&mut magic_number) {
            return magic_number == MAGIC_NUMBER;
        }
    }
    
    false
}
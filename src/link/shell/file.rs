use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// 创建文件并写入初始内容（如果文件已存在，则会覆盖）
pub fn create_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    println!("文件已创建: {}", file_path);
    Ok(())
}

/// 删除文件
pub fn delete_file(file_path: &str) -> io::Result<()> {
    if Path::new(file_path).exists() {
        fs::remove_file(file_path)?;
        println!("文件已删除: {}", file_path);
    } else {
        println!("文件不存在: {}", file_path);
    }
    Ok(())
}

/// 覆盖文件内容
pub fn overwrite_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    println!("文件内容已覆盖: {}", file_path);
    Ok(())
}

/// 追加内容到文件
pub fn append_to_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)?;
    file.write_all(content.as_bytes())?;
    println!("内容已追加到文件: {}", file_path);
    Ok(())
}

/// 读取文件内容并返回字符串
pub fn read_file(file_path: &str) -> io::Result<String> {
    if Path::new(file_path).exists() {
        let content = fs::read_to_string(file_path)?;
        println!("文件内容已读取: {}", file_path);
        Ok(content)
    } else {
        println!("文件不存在: {}", file_path);
        Err(io::Error::new(io::ErrorKind::NotFound, "文件不存在"))
    }
}
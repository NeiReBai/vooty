use std::fs;
use std::io::{self, Write};
use std::path::Path;

mod file;

fn create_directory(path: &str) -> io::Result<()> {
    // 创建文件夹
    fs::create_dir_all(path)?;
    // println!("文件夹已成功创建: {}", path);
    Ok(())
}

pub fn init() {
    let config_path = Path::new("vottyConfig.json");

    if config_path.exists() {
        println!("·当前项目已经完成了初始化，如需继续初始化，请删除“vottyConfig.json”后再次执行当前命令");
    } else {
        let mut file = fs::File::create(config_path).expect("无法创建文件");
        file.write_all(b"{\n  \"cache\":[],\n  \"source\": \"https://plumlanguage.github.io/vottySource/\"\n}")
            .expect("无法写入到文件");
        println!("初始化成功：已创建 'vottyConfig.json'");
        let folder_path = "./Votty";

        match create_directory(folder_path) {
            Ok(()) => { 
                println!("操作完成");
                // 创建文件并写入内容
                let file_path = "Votty/mod.rs";

                if let Err(e) = file::create_file(file_path, "") {
                    eprintln!("创建文件失败: {}", e);
                }

            },
            Err(e) => eprintln!("创建文件夹失败: {}", e),
        }

    }
}

pub fn add_to_mod_file(mode_name: &str){
    // 追加内容到文件
    let file_path = "Votty/mod.rs";
    let add_mode_language = format!("pub mod {}{}", mode_name, ";\n");
    if let Err(e) = file::append_to_file(file_path, &add_mode_language) {
        eprintln!("追加内容失败: {}", e);
    }

}
mod link;

use std::env;




fn main() {
    let config_file = "vottyConfig.json";
 
    let args: Vec<String> = env::args().collect();
    let x: Vec<&str> = args.iter().flat_map(|arg| arg.split_whitespace()).collect();
    println!("Split command line arguments: {:?}", x);
    
    if x[1] == "help"{
        println!("当前没有可用的帮助❓：（");
    }else if x[1] == "init"{
        link::shell::init()
    }
    // 功能1：添加依赖库
    else if x[1] == "add"{
        if let Err(e) = link::json::add_to_cache(config_file, x[2].to_string()) {
            eprintln!("添加依赖失败❌: {}", e);
        } else {
            println!("已成功添加依赖✅");
            link::shell::add_to_mod_file(x[2])
        }
    }
    // 功能2：删除依赖库
    else if x[1] == "rem"{
        if let Err(e) = link::json::remove_from_cache(config_file, x[2].to_string()){
            eprintln!("删除依赖失败❌: {}", e);
        }else{
            println!("删除依赖成功✅")
        }
    }
    // 功能3：更新源
    else if let Err(e) = link::json::update_source(config_file, "https://new-source.example.com".to_string()) {
        eprintln!("更新 source 失败❌: {}", e);
    } else {
        println!("已成功更新 source✅");
    }
}
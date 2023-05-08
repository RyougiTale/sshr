// 引入需要的库
use clap::{App, Arg, SubCommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::process::Command;
// 定义一个结构体来存储代理映射
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
struct ProxyMapping {
    src_port: String,
    target_ip: String,
    target_port: String,
}

// 从配置文件中读取目标 IP 和源 IP
// fn get_config() -> HashMap<String, String> {
//     // 从配置文件读取配置（例如 config.toml）
//     // 返回 HashMap
// }

// 列出所有代理映射
fn list_proxies() {
    // 从映射文件读取代理映射
    // 打印代理映射
}

// 设置目标 IP 或源 IP
fn set_ip(key: &str, value: &str) {
    // 更新配置文件中的目标 IP 或源 IP
}

// 为给定的端口创建新的代理映射
fn create_proxy(src_port: &str, target_port: &str) {
    // 从配置文件中读取目标 IP 和源 IP
    // 检查是否已经存在相同的代理映射
    // 构建 SSH 命令
    // 使用 std::process::Command 执行 SSH 命令
    // 将新的代理映射添加到映射文件中
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    src_ip: String,
    target_ip: String,
}

fn read_config() -> Config {
    let config_file = fs::File::open("config.json").expect("Unable to open config file");
    let config: Config = serde_json::from_reader(config_file).expect("Unable to read config file");
    config
}

fn write_config(config: &Config) {
    let json = serde_json::to_string_pretty(config).expect("Unable to convert config to JSON");
    let mut file = fs::File::create("config.json").expect("Unable to create config file");
    file.write_all(json.as_bytes())
        .expect("Unable to write config to file");
}

fn parse_ip_port(input: &str) -> (String, String) {
    let parts: Vec<&str> = input.split(':').collect();
    let ip = if parts.len() == 2 {
        parts[0]
    } else {
        "localhost"
    };
    let port = parts.last().unwrap();
    (ip.to_string(), port.to_string())
}

fn main() {
    let app = App::new("sshr")
        .version("1.0")
        .author("ryougi")
        .about("SSH reverse proxy management tool")
        .subcommand(SubCommand::with_name("ls").about("List current port mappings"))
        .subcommand(
            SubCommand::with_name("set_target_ip")
                .about("Set target IP address")
                .arg(
                    Arg::with_name("TARGET_IP")
                        .help("Target IP address")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("set_src_ip")
                .about("Set source IP address")
                .arg(
                    Arg::with_name("SRC_IP")
                        .help("Source IP address")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("proxy")
                .about("Create a new reverse proxy")
                .arg(
                    Arg::with_name("SRC")
                        .help("Source in format IP:PORT or PORT")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("TARGET")
                        .help("Target in format IP:PORT or PORT")
                        .required(true)
                        .index(2),
                ),
        );

    // // 读取配置
    // let config = read_config();
    // println!("{:?}", config);

    // // 修改配置
    // let new_config = Config {
    //     src_ip: "127.0.0.1".to_string(),
    //     target_ip: "".to_string(),
    // };

    // // 保存配置
    // write_config(&new_config);
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("ls", _sub_m)) => {
            // 调用 list_proxies()
            println!("Listing current port mappings...");
        }
        Some(("set_target_ip", sub_m)) => {
            let target_ip = sub_m.value_of("TARGET_IP").unwrap();
            // 调用 set_ip("target_ip", target_ip)
            println!("Setting target IP: {}", target_ip);
        }
        Some(("set_src_ip", sub_m)) => {
            let src_ip = sub_m.value_of("SRC_IP").unwrap();
            // 调用 set_ip("src_ip", src_ip)
            println!("Setting source IP: {}", src_ip);
        }
        Some(("proxy", sub_m)) => {
            let src = sub_m.value_of("SRC").unwrap();
            let target = sub_m.value_of("TARGET").unwrap();

            let (src_ip, src_port) = parse_ip_port(src);
            let (target_ip, target_port) = parse_ip_port(target);
            // 调用 create_proxy(src_port, target_port)
            println!(
                "Creating a new reverse proxy: {} -> {}",
                src_port, target_port
            );
        }
        _ => {
            // 当不匹配任何子命令时显示帮助信息
            // app.lock().unwrap().print_help();
            println!("not match");
        }
    }
}

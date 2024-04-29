use std::{env, fs, process};
use std::error::Error;

fn main() {
    // env::args()用于获取程序运行时传递给它的命令行参数
    // 比如cargo run -- -path something
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("错误参数:{err}"); // 如果传入的参数不符合预期
        process::exit(1); // 退出程序
    });

    println!("第一个参数，暂未用到:{:?}", config.query);
    println!("第二个参数，用于在run()里读取：{:?}", config.file_path);

    // 示例window:    cargo run something E:/1.txt
    run(config).unwrap_or_else(|err| {
        println!("读取错误:{err}");
    });
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    println!("文件内容：{content}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // 构建数据，并检查
    // 这里为什么要使用static的生命周期？并且返回的不是Err，而是str？
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数不足");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
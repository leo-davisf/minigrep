use std::env; //env::args方法会读取并分析传入的命令行参数
use std::process;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    //debug!(args);  //输出读取到的数组内容，第一个为程序的可执行路径名
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Search for {}", config.query);
    println!("In file {}", config.file_path);
    
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}






use std::env; //env::args方法会读取并分析传入的命令行参数
use std::fs;
use std::process;  //
fn main() {
    let args: Vec<String> = env::args().collect();
    //debug!(args);  //输出读取到的数组内容，第一个为程序的可执行路径名
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Search for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the  file");

    println!("With text:\n{contents}");
}

struct  Config<'a> {
    query: &'a str,
    file_path: &'a str
}

impl<'a> Config<'a> {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = &args[1];
        let file_path = &args[2];
    
        Ok(Config { query, file_path })
    }
}
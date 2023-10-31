use std::fs;
use std::error::Error;
pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");  //错误处理统一实现
        }

        let query = &args[1];
        let file_path = &args[2];
    
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the  file");

        println!("With text:\n{contents}");

        Ok(())
}
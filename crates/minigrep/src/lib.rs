use std::{env, error::Error, fs::File, io::Read};

///
/// 配置
///
pub struct Config {
    ///
    /// 查询字符串
    ///
    query: String,

    ///
    /// 文件名
    ///
    filename: String,

    ///
    /// 是否大小写不敏感的
    ///
    case_insensitive: bool,
}

impl Config {
    ///
    /// 创建Config实例
    ///
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_insensitive,
        })
    }

    ///
    /// 获取查询字符串
    ///
    pub fn query(&self) -> &str {
        &self.query
    }

    ///
    /// 获取文件名
    ///
    pub fn filename(&self) -> &str {
        &self.filename
    }
}

///
/// 执行
///
pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let mut content = String::new();
    File::open(cfg.filename)?.read_to_string(&mut content)?;

    let result = if cfg.case_insensitive {
        search_case_insensitive(&cfg.query, &content)
    } else {
        search(&cfg.query, &content)
    };

    for line in result {
        println!("{}", line);
    }

    println!("Done!");
    Ok(())
}

///
/// 查询包含指定字符串的行
///
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vec = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            vec.push(line.trim());
        }
    }
    vec
}

///
/// 查询包含指定字符串的行
/// - 大小写不敏感的
///
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = vec![];
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line.trim());
        }
    }
    result
}

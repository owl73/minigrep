use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents) 
    } else {
        search_case_insensitive(&config.query, &contents) 
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_failure() {
        let a = String::from("a");
        let b = String::from("b");
        let test_args = vec![a, b];
        match Config::new(&test_args){
            Ok(_) => assert!(false, "Config constructor accepts failure"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn normal_run() {
        let pattern = String::from("body");
        let filename = String::from("poem.txt");

        let config = Config::new(
            &vec![String::from("blank"), pattern, filename]
        ).unwrap();

        match run(config) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false, "Make sure poem.txt exists in root folder"),
        }
    }

    #[test]
    fn bad_file() {
        let pattern = String::from("body");
        let filename = String::from("delete_me.txt");

        let config = Config::new(
            &vec![String::from("blank"), pattern, filename]
        ).unwrap();

        match run(config) {
            Ok(_) => assert!(false, "Make sure delete_me.txt does not exist"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents),
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents),
        );
    }

}

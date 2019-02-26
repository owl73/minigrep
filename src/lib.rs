use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename})
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
    fn run_logic() {
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
    fn run_false_file() {
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
    fn one_search_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents),
        );
    }
}

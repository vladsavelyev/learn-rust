use std::env;
use std::error::Error;
use std::fs;

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&cfg.file_path)?;

    let results = if cfg.ignore_case {
        search_insensitive(&contents, &cfg.query)
    } else {
        search(&contents, &cfg.query)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(text: &'a str, query: &str) -> Vec<&'a str> {
    text.lines()
        .filter(|x| x.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(text: &'a str, query: &str) -> Vec<&'a str> {
    text.lines()
        .filter(|x| x.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "dear";
        let text = "\
Hello
Dear
you are my dear";
        assert_eq!(vec!["you are my dear"], search(text, query));
    }

    #[test]
    fn case_insensitive() {
        let query = "dEaR";
        let text = "\
Hello
Dear
you are my dear";
        assert_eq!(
            vec!["Dear", "you are my dear"],
            search_insensitive(text, query)
        );
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("not enough arguments"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("not enough arguments"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

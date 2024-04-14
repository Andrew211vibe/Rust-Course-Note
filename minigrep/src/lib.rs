use std::{fs, env, error::Error};

pub struct Config<'a, 'b> {
    pub query: &'a str,
    pub file_path: &'b str,
    pub ignore_case: bool,
}

impl<'a, 'b> Config<'a, 'b> {
    pub fn new<'c>(args: &'c [String]) -> Result<Self, &'static str>
        where 'c: 'a + 'b
    {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }

        let query = &args[1];
        let file_path = &args[2];
        let ignore_case = match env::var("IGNORE_CASE") {
            Ok(val) if val == "1" => true,
            _ => match args.get(3) {
                Some(flag) => flag == "1",
                _ => false,
            }
        };
        
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let res = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in res {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut res = vec![];
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         res.push(line);
    //     }
    // }
    // res
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let query = query.to_lowercase();
    // let mut res = vec![];
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         res.push(line);
    //     }
    // }
    // res
    contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
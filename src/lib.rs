use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    
    let result = if config.case_sensitive {
        search(&config.query, &content) 
    }else{
        search_case_insensitive(&config.query, &content) 
    };

    println!("{:?}", result);
        
    Ok(())
}
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool, 
} 

impl  Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get the query string"),
        };
        
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get the filename string"),
        };
        
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        Ok(Self {
            query,
            filename,
            case_sensitive,

        })    
    }
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}


pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for i in contents.lines() {
        if i.to_uppercase().contains(&query.to_uppercase()) {
            results.push(i);
        }
    }
    results
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    


    #[test]
    fn case_insensitive(){
        let query = "RusT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
trust me.";

        assert_eq!(vec!["Rust:", "trust me."], search_case_insensitive(query, contents));
    }
}
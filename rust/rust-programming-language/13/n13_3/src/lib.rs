use::std::{env};

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

// impl Config {
//     pub fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         let ignore_case = env::var("IGNORE_CASE").is_ok();

//         Ok(Config, {
//             query,
//             file_path,
//             ignore_case,
//         })
//     }
// }

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config, {
            query,
            file_path,
            ignore_case,
        })
    }

    // pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //     let mut results = Vec::new();

    //     for line in contents.lines() {
    //         if line.contains(query) {
    //             results.push(line);
    //         }
    //     }

    //     results
    // }

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }
}
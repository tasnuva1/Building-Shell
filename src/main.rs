use anyhow::{Context, Result};
use std::io::{self, BufRead, Read};
use std::io::{stdin, stdout, Cursor, Write};

fn main() -> Result<()> {
    println!("Hello, world!");
    loop {
        print!("$ ");
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .context("Did not enter a correct string");
        // match s.as_str() {
        //     // "exit" => ,
        //     "" => println!("{}: command not found", s.trim()),
        //     _ => {

        //     }
        // }

        let cursor = Cursor::new(s.as_bytes());
        let mut split_iter = cursor.split(b' ').map(|v| v.unwrap());

        let mut arguments_vec = Vec::new();
        let args = split_iter
            .into_iter()
            .map(|v| String::from_utf8(v).unwrap());

        args.into_iter().for_each(|e| arguments_vec.push(e));

        match arguments_vec[0].as_str().trim() {
            "exit" => {
                if arguments_vec.len() == 2 {
                    match arguments_vec[1].trim().parse::<i32>() {
                        Ok(num) => std::process::exit(num),
                        Err(_) => println!("Please enter a number in exit code argument"),
                    };
                } else if arguments_vec.len() == 1 {
                    std::process::exit(0);
                }
            }
            "" => println!("Please enter a command"),
            _ => println!("{}: command not found", arguments_vec[0].trim()),
        }
        // println!("{:?}", arguments_vec[0].trim());
    }
    // Ok(())
}

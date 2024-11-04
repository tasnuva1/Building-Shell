use anyhow::{Context, Result};
// use std::collections::binary_heap;
use std::env;
use std::io::BufRead;
use std::io::{stdin, stdout, Cursor, Write};

fn main() -> Result<()> {
    loop {
        print!("$ ");
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .context("Did not enter a correct string")
            .ok();

        let cursor = Cursor::new(s.as_bytes());
        let split_iter = cursor.split(b' ').map(|v| v.unwrap());

        let mut arguments_vec = Vec::new();
        let args = split_iter.map(|v| String::from_utf8(v).unwrap());

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
            "echo" => {
                if arguments_vec.len() >= 2 {
                    let mut stdout = std::io::stdout().lock();

                    arguments_vec[1..].into_iter().for_each(|e| {
                        stdout.write(e.as_bytes()).context("stdout write error");
                        stdout.write(b" ").context("stdout write error");
                    });
                } else if arguments_vec.len() == 1 {
                    println!();
                }
            }
            "type" => {
                if arguments_vec.len() >= 2 {
                    let key = "PATH";
                    let mut founded = Vec::new();
                    let mut not_found_bool = Vec::new();
                    // for (_i, e) in argument_vec_copy[1..].iter().enumerate() {
                    arguments_vec[1..].iter().for_each(|e| {
                        let e = e.trim();
                        if e == "exit" {
                            println!("exit is a shell builtin");
                            founded.push(e);
                        } else if e == "echo" {
                            println!("echo is a shell builtin");
                            founded.push(e);
                        } else if e == "type" {
                            println!("type is a shell builtin");
                            founded.push(e);
                        }

                        match env::var_os(key) {
                            Some(paths) => {
                                for path in env::split_paths(&paths) {
                                    if path.is_dir() {
                                        for entry in std::fs::read_dir(path).unwrap() {
                                            // let entry = entry?;
                                            let path = entry.as_ref().unwrap().path();
                                            if path.file_name().unwrap().to_str().unwrap()
                                                == e.trim()
                                            {
                                                println!(
                                                    "{} is {}", // TODO: last element you don't want \n
                                                    e.trim(),
                                                    path.to_str().unwrap()
                                                );

                                                founded.push(e.trim());
                                            } else {
                                            }
                                        }
                                    }
                                }
                            }

                            None => println!("{key} is not defined in the environment."),
                        }

                        founded.iter().for_each(|f| {
                            if e.trim() != *f {
                                not_found_bool.push(true);
                            } else {
                                not_found_bool.push(false);
                            }
                        });
                        let truee = not_found_bool.iter().all(|e| *e == true);
                        if truee == true {
                            println!("{} not found", e.trim());
                        }
                        not_found_bool.clear();
                    })
                } else if arguments_vec.len() == 1 {
                    println!();
                }
            }
            "" => println!("Please enter a command"),
            _ => println!("{}: command not found", arguments_vec[0].trim()),
        }
    }
}

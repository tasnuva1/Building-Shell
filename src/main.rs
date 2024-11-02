use anyhow::{Context, Result};
use std::collections::binary_heap;
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
                    // stdout.write_all(b"some bytes").unwrap();
                    // println!("{:?}", arguments_vec);
                } else if arguments_vec.len() == 1 {
                    println!();
                }
            }
            "type" => {
                if arguments_vec.len() >= 2 {
                    arguments_vec[1..].into_iter().for_each(|e| {
                        let e = e.trim();
                        if e == "exit" {
                            println!("exit is a shell builtin")
                        } else if e == "echo" {
                            println!("echo is a shell builtin")
                        } else if e == "type" {
                            println!("type is a shell builtin")
                        } else {
                            // println!("{}: not found", e)
                        }
                    });

                    let key = "PATH";
                    let mut result_path = String::new();
                    // let mut this_time_found = false;
                    let mut founded = Vec::new();
                    match env::var_os(key) {
                        Some(paths) => {
                            arguments_vec[1..].iter().for_each(|file| {
                                for path in env::split_paths(&paths) {
                                    if path.is_dir() {
                                        for entry in std::fs::read_dir(path).unwrap() {
                                            // let entry = entry?;
                                            let path = entry.as_ref().unwrap().path();
                                            if path.file_name().unwrap().to_str().unwrap()
                                                == file.as_str().trim()
                                            {
                                                result_path.push_str(file.as_str().trim());
                                                result_path.push_str(" is ");
                                                result_path.push_str(path.to_str().unwrap());
                                                result_path.push_str("\n");

                                                // println!(
                                                //     "{} is {}",
                                                //     file.as_str().trim(),
                                                //     path.to_str().unwrap()
                                                // );
                                                // this_time_found = true;
                                                // println!("{}", this_time_found);
                                                founded.push(file.as_str().trim());
                                            } else {
                                                // println!("{} not found", file.as_str().trim());
                                                // println!("{}", this_time_found);
                                                // this_time_found = false;
                                            }
                                        }
                                    }
                                } // down

                                // if this_time_found == false {
                                //     println!("{} not found-----", file.as_str().trim());
                                // } else {
                                //     println!("{}", result_path);
                                // } ////////
                                // match result_path.as_str() {
                                //     "" => println!("{} not found", file.as_str().trim()),
                                //     _ => println!("{}", result_path),
                                // }
                            }); //
                                // arguments_vec[1..].iter().for_each(|file| {
                                //     for (i, e) in founded.iter() {
                                //         if file.as_str().trim() == e.trim() {
                                //             // println!("{} not found-----", file.as_str().trim());
                                //         } else {
                                //             println!("{} not found-----", file.as_str().trim());
                                //         }
                                //     }
                                // });
                                // arguments_vec.iter().filter()

                            println!("{}", result_path);
                            println!("{:?}", founded);
                            println!("{:?}", arguments_vec);
                        }

                        None => println!("{key} is not defined in the environment."),
                    }
                    // println!("{:?}", result_path);
                    // match result_path.as_str() {
                    //     "" => println!("{} not found", arguments_vec[1].trim()),
                    //     _ => println!("{}", result_path),
                    // }
                } else if arguments_vec.len() == 1 {
                    println!();
                }
            }
            "" => println!("Please enter a command"),
            _ => println!("{}: command not found", arguments_vec[0].trim()),
        }
        // println!("{:?}", arguments_vec[0].trim());
    }
    // Ok(())
}

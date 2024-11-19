#[allow(unused_imports)]
use std::{env, fmt::format, process::{Command, Stdio}};
use std::{fs::{remove_file, File}, num::ParseIntError};
use std::io::prelude::*;
use crate::Pair;
use crate::Rule;

pub fn parse_rust_expression(s: &str) -> Option<u32> {
    let code = format!(
        r#"
            fn main() {{
                let r: i32 = {{ {} }};
                println!("{{}}", r);
            }}
            "#, s
        );

    let mut temp_file_path = env::current_dir().unwrap();
    temp_file_path.push("__temp_program.rs");
    
    let mut x: Option<u32> = None;

    if let Ok(mut file) = File::create(&temp_file_path) {
        if file.write_all(code.as_bytes()).is_ok() {
            
            // Compile the code
            let binary_path = temp_file_path.with_extension("");
            let rustc_status = Command::new("rustc")
                .arg(&temp_file_path)
                .arg("-o")
                .arg(&binary_path)
                .status();

            if let Ok(status) = rustc_status {
                if status.success() {
                    // Run the compiled binary and capture output.
                    let output = Command::new(&binary_path)
                        .stdout(Stdio::piped())
                        .output()
                        .expect("Failed to execute binary.");
                    let sval =  String::from_utf8_lossy(&output.stdout).into_owned();
                    x = match sval.trim().parse::<u32>() {
                        Ok(val) => Some(val),
                        Err(_) => {
                            println!("Code `{}` evaluated to: {} , but cannot parse to u32.\n", s, sval);
                            None
                        }
                    };
                } 
                else {
                    eprintln!("Error: rustc failed to compile the temporary `rust!(...)` code.");
                }
            } 
            else {
                eprintln!("Error: failed to execute the temporary `rust!(...)` code.");
            }
        } 
        else {
            eprintln!("Error: cannot write to the temporary file to execute `rust!(...)` code.");
            eprintln!("Code to be evaluated:\n{}", code);
        }
        
        // Clean up the temporaries.
        remove_file(temp_file_path).unwrap();
    }
    else {
        eprintln!("Code to be evaluated:\n{}", code);
        eprintln!("Error: cannot create a temporary file. Write permissions allowed? :(");
    }

    x
}

/* Must match a Pair produced from NUM rule. */
pub fn parse_number(p: Pair<'_, Rule>) -> Option<u32> {
    assert_eq!(p.as_rule(), Rule::NUM);

    let inside = p.into_inner().next().unwrap();
    match inside.as_rule() {
        Rule::DEC => inside.as_str().parse().map_err(|x| { eprintln!("Failed parsing string: {} as base-10 number. Number too large?\n", inside.as_str()); x }).ok(),
        Rule::HEX => u32::from_str_radix(&inside.as_str()[2..], 16).map_err(|x| { eprintln!("Failed parsing string: {} as hex. Number too large?\n", &inside.as_str()[2..]); x }).ok(),
        Rule::BIN => u32::from_str_radix(&inside.as_str()[2..], 2).map_err(|x| { eprintln!("Failed parsing string: {} as binary. Number too large?\n", &inside.as_str()[2..]); x }).ok(),
        Rule::RUST => parse_rust_expression(inside.as_str()),
        _ => unreachable!(),
    }
}


#![allow(unused_imports)]
use std::env;
// use std::fmt::Display;
use std::io::{self, stdout, Result as IOResult, Write};
// use std::string::FromUtf8Error;

mod osc;
use osc::cb::{ClipboardRead, ClipboardWrite};

// available sub-commands
const SUB_CMDS: [&str; 5] = ["get", "set", "clear", "hist", "help"];

fn main() -> IOResult<()> {
    println!("Hello, world!");

    /*
    let mut stdin = io::stdin();
    let _ = stdin.lock();
    let mut stdout = io::stdout();
    let _ = stdout.lock();
    */

    let cl_input: Vec<String> = env::args().skip(1).collect();

    let first_arg: &str = cl_input[0].as_str();

    dbg!(&cl_input);
    dbg!(&first_arg);

    let has_subcmd: bool = SUB_CMDS.contains(&first_arg);

    if !has_subcmd {
        match first_arg {
            "-h" | "--help" => println!("Help info..."),
            "-v" | "--version" => println!("Version info..."),
            i if i.starts_with("-") => println!("Unsupported flag"),
            _ => println!("Unsupported subcommand"),
        }
        return Ok(());
    };

    let print_red_txt = |x: &str| format!("\x1b[38;5;1m{}\x1b[m", x);
    println!("{}", print_red_txt("Texting red / error text"));

    match first_arg {
        "get" => (),
        "set" => (),
        "clear" => (),
        "history" => (),
        "help" => (),
        _ => println!("{}", print_red_txt("( how tf did this happen ?!?! )")),
    }

    Ok(())
}

#[allow(dead_code, unused_variables)]
fn get_cb(reg: Option<char>, _flags: Option<&[String]>) -> usize {
    // ClipboardRead
    todo!()
}

#[allow(dead_code, unused_variables)]
fn set_cb(reg: Option<char>, message: &String, flags: Option<&[String]>) -> usize {
    // ClipboardWrite
    // if flags.contains("-a" or "--append"), then append (also needs ClipboardRead)
    todo!()
}

#[allow(dead_code, unused_variables)]
fn clear_cb(reg: Option<char>, flags: Option<&[String]>) -> usize {
    // ClipboardWrite
    // alias for set_cb(_, &"".to_string, None)
    todo!()
}

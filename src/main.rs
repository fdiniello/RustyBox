
use std::env;

mod list;
mod basename;

fn main() {
    let mut cmd: Vec<String> = env::args().collect();
    let args = if cmd.len() == 1 {
        vec![] //args = empty vector
    } else {
        cmd.split_off(1) // args = remaining main arguments
    };
    println!("cmd: {:?}", cmd);
    println!("args: {:?}", args);

    let cmd = basename::basename( cmd );

    match cmd.as_str() {
        "RustyBox"|"rusty_box" => {
            println!("this is RustyBox!");
        },
        "basename"=> {
            println!( "{}", basename::basename( args ) );
        }
        "list" | "ls" => {
            println!( "{}", list::ls( args ) );
        },
        _ => {
            println!("{} not implemented!",cmd);
        }
    }
}
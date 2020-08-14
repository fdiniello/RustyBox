
use std::env;

mod list;
mod link;
mod basename;

fn not_implemented( _args: Vec<String> ) -> Result<String,String> {
    let cmd: Vec<String> = env::args().collect();
    let str = format!("{} command not implemented yet", cmd[0]);
    Err(str)
}

fn rusty_box( _args: Vec<String> ) -> Result<String,String> {
    Ok("this is RustyBox!".to_string())
}

fn main() {
    let mut cmd: Vec<String> = env::args().collect();

    let args = if cmd.len() == 1 {
        vec![] //args = empty vector
    } else {
        cmd.split_off(1) // args = remaining main arguments
    };

    if let Ok(cmd) = basename::basename( cmd ){
        let cmd = match cmd.as_str() {
            "RustyBox"|"rusty_box" => rusty_box,
            "basename" => basename::basename,
            "list" | "ls" => list::ls,
            "link" | "ln" => link::ln,
            _ => not_implemented
            };
            let res = cmd(args);
            if let Ok(result) = res {
                println!("{}", result);
            } else if let Err(what) = res {
                panic!(what);
            }
    }

}

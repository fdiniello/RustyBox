use std::path::Path;
use std::fmt::Write;
use std::fs;


pub fn ls( args: Vec<String> ) -> String {
    let mut ret = String::new();
    let dir = if args.len() == 0 {
        Path::new( "./" )
    } else {
        Path::new( &args[ args.len() - 1 ] )
    };

    if let Ok(entries) = fs::read_dir( dir ) {
        for each in entries {
            if let Ok(each) = each {
                if let Ok(name) = each.file_name().into_string() {
                    write!(ret,"{}  ", name);
                }
            }
        }
        println!();
    }
    ret
}

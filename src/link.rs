

use crate::basename::basename;

pub fn ln( args: Vec<String>) -> Result<String,String> {

    let link_fn = if find_arg( &args, 's' ) {
        std::os::unix::fs::symlink
    } else {
        std::fs::hard_link
    };


    let src =
    if let Some(src) = find_src( &args ) {
        src
    } else {
        return Err(String::from("Can't find src"));
    };

    let dst = if let Some(dst) = find_dst( &args, &src){
        dst
    } else {
        return Err(String::from("Can't find dst"));
    };


    let res = link_fn(src,dst);


    if let Ok(_m) = res {
        Ok( String::from("") )
    } else if let Err(e) = res {
        Err( e.to_string() )
    } else {
        Err( String::from("") )
    }
}
// busco si aparece S o no para definir hard/soft
fn find_arg(args: &Vec<String>, arg : char ) -> bool {
    for each in args {
        if each.as_bytes()[0] != b'-' {
            continue;
        }
        for c in each.chars() {
            if c == arg {
                return true;
            }
        }
    }
    false
}
// busca el primer argumento que no empiece con -
fn find_src( args: &Vec<String>) -> Option<String> {
    for s in args {
        if s.as_bytes()[0] != b'-' {
            return Some( s.clone() );
        }
    }
    None
}


fn find_dst( args: &Vec<String>, src: &String ) -> Option<String> {
    if let Some(last) = args.last() {
        if last != src {
            Some(last.to_string())
        } else {
            if let Ok(dst) = basename( vec![src.clone()] ){
                Some(dst)
            } else {
                None
            }
        }
    } else {
        None
    }
}

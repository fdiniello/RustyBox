pub fn basename(mut args: Vec<String>) -> String {
    //default mode: only 1 arg
    if args.len() == 1 {
        let mut root_offset = args[0].find('/');
        while root_offset.is_some() {
            args[0] = args[0].split_off( root_offset.unwrap() + 1 );
            root_offset = args[0].find('/');
        }
        args[0].clone()
    }else {
        // others: not implemented
        args[0].clone()
    }
}

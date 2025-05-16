use std::path::PathBuf;

fn main() {
    let (source_code, dest_path) = parse_args();
}

fn parse_args() -> (String, PathBuf) {
    let mut args = std::env::args();
    if args.len() < 3 {
        panic!("Only got {} arguments but required 3.", args.len());
    }

    // skip the program name
    args.next().unwrap();

    let source_code = {
        let source_path = args.next().unwrap();
        std::fs::read_to_string(source_path).unwrap()
    };

    (source_code, PathBuf::from(args.next().unwrap()))
}

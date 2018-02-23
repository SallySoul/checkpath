use std::env;
use std::path::PathBuf;

fn main() {
	let argv : Vec<String> = env::args().collect();
    let mut path_buf = PathBuf::from(&argv[1]);

    let mut keep_going = true;
    while keep_going {
        if path_buf.exists() {
            print!("✔");
            keep_going = false;
        } else {
            print!("✘");
        }

        println!(" {}", path_buf.display());

        keep_going = path_buf.pop() && keep_going;
    }
}

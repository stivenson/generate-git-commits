extern crate generate_commits;
extern crate run_script;

use std::path::Path;

fn main() {
    println!("Path of repository to check: {:#?}", std::env::args().nth(1).expect("no path given"));
    let path = std::env::args().nth(1).expect("no path given");
    let path_object = Path::new(&path);
    println!("{:#?}", generate_commits::generator::run(&path_object));
    println!("The End");
}

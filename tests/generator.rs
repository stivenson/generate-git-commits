extern crate generate_commits;
use std::path::Path;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore]
    fn test_run_process() {
        let path_object = Path::new(&"<PLEASE, HERE REPLACE BY ABSOLUTE PATH TO ANY PROJECT>");
        let res = generate_commits::generator::run(&path_object);
        let mut process_message = String::from("");
        match res {
            Ok(s) => {
                process_message = s;
            },
            Err(_) => {
                println!("Description error in test_run_process")
            }
        }
        println!("Result of test_run_process: {:?}", &process_message);
        assert_eq!(&process_message, &"Process finalized");
    }

    
    #[test]
    #[should_panic(expected = "The path that you sent is empty")]
    fn test_run_process_with_empty_path() {
        let path_object = Path::new(&"");
        let res = generate_commits::generator::run(&path_object);
        let mut process_message = String::from("");
        match res {
            Ok(s) => {
                process_message = s;
            },
            Err(_) => {
                println!("Description error in test_run_process")
            }
        }
        println!("Result of test_run_process: {:?}", &process_message);
    }
}

// Use the command "cargo test -- --nocapture" to check the "println!" returns (it's optionally)

// @TODO: pending add more tests.

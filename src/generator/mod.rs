extern crate chrono;
extern crate run_script;

use std::path::Path;
use std::io::Result;
use chrono::{DateTime, UTC};
use crate::generator::chrono::TimeZone;
use run_script::ScriptOptions;
use std::fs::{self, DirEntry};
use std::time::{SystemTime, UNIX_EPOCH};

const QUOTATION_MARK: &str = "\"";

macro_rules! result_commit {
    ($errors:expr , $code:expr , $output:expr ) => (
        println!("Errors: {:?}", $errors);
        println!("Exit Code: {:?}", $code);
        println!("Output: {:?}", $output);
    )
}
// DOC DEVELOPERS: more info about designators to macros: https://doc.rust-lang.org/rust-by-example/macros/designators.html

trait Script {

    // @TODO pending init big_commands variables

    fn result(&self,code: i32,output: String, error: String) -> (&i32, &String, &str);

    fn run_git_command(&self, big_commands: &str) {
        let mut options = ScriptOptions::new();
        options.runner = None; // The script runner, for example bash. By default for windows it's cmd.exe and for other systems it is sh.
        options.capture_output = true; // True to capture and return the output. False will print it to the parent process output.
        options.exit_on_error = false; // Adds set -e option (not available for windows)
        options.print_commands = false; // Adds set -x option (not available for windows)
        let args = vec![];
        let (code, output, error) = run_script::run(
            &format!(r#" {} "#, &big_commands).to_string(),
            &args,
            &options
        ).unwrap();
        self.result(code, output, error);
    }
}

struct Commit {
    pub relative_path_file: String,
    pub path_project: String,
    pub message: String,
    pub date_commit: String,
    pub result: (i32, String, str)
}

impl Script for Commit {
    fn result(&self, code: i32,output: String, error: String) -> (&i32, &String, &str) {
        let mut error: &str = "Without Errors :)";
        if &self.result.2 != "" {
            error = &self.result.2;
        }
        return (&self.result.0, &self.result.1, &error);
    }
    // @TODO pending logic to create big command
}


fn system_time_to_date_time(t: &SystemTime) -> DateTime<UTC> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => {
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        },
    };
    UTC.timestamp(sec, nsec)
}

fn process_dirs(dir: &Path, cb: &Fn(&Path, &Path, &DirEntry)) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                process_dirs(&path, cb)?;
            } else {
                cb(&dir, &path, &entry);
            }
        }
    }
    Ok({})
}

fn manage_dirs(path_project: &Path, dir: &Path, entry: &DirEntry) {
    if let Ok(metadata) = entry.metadata() {
        let system_time = metadata.modified();
        match system_time {
            Ok(n) => {
                let date_time = system_time_to_date_time(&n);
                let exe = execute(&path_project.display().to_string(), &dir.display().to_string(), date_time.to_string());
                match exe {
                    Ok(_n) => {
                        println!("Completed iteration");
                    },
                    Err(_) => panic!("Unknown error in iteration"),
                }
            },
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
        
    } else {
        println!("Couldn't get metadata for {:?}", entry.path());
    }
}

fn execute(path_project: &String, relative_path_file: &String, date_commit: String) -> Result<()> {
    let message = "refactor: add file ".to_string() + &relative_path_file.to_owned();
    let big_commands = "cd ".to_owned()+&format!("{}{}{}",&QUOTATION_MARK, &path_project, &QUOTATION_MARK).to_string()+&" && ( git add ".to_owned()+&format!("{}{}{}",&QUOTATION_MARK, &relative_path_file, &QUOTATION_MARK).to_string()+&" || true ) && ( git commit -m ".to_owned()+&format!("{}{}{}",&QUOTATION_MARK, &message, &QUOTATION_MARK).to_string()+&" || true ) && ( git commit --amend --no-edit --date ".to_owned()+&format!("{}{}{}", &QUOTATION_MARK, &date_commit, &QUOTATION_MARK).to_string()+&" || true )".to_owned();
    let mut options = ScriptOptions::new();
    options.runner = None; // The script runner, for example bash. By default for windows it's cmd.exe and for other systems it is sh.
    options.capture_output = true; // True to capture and return the output. False will print it to the parent process output.
    options.exit_on_error = false; // Adds set -e option (not available for windows)
    options.print_commands = false; // Adds set -x option (not available for windows)

    let args = vec![];
    let (code, output, error) = run_script::run(
        &format!(r#" {} "#, &big_commands).to_string(),
        &args,
        &options
    ).unwrap();
    result_commit!(error, code, output);
    Ok({})
}

fn init_repo (path_object: &Path) {
    let init_commands = "cd ".to_owned()+&format!("{}{}{}",&QUOTATION_MARK, &path_object.display().to_string(), &QUOTATION_MARK).to_string()+&" && rm -rf .git && git init ".to_owned();
    let mut options = ScriptOptions::new();
    options.runner = None; // The script runner, for example bash. By default for windows it's cmd.exe and for other systems it is sh.
    options.capture_output = true; // True to capture and return the output. False will print it to the parent process output.
    options.exit_on_error = false; // Adds set -e option (not available for windows)
    options.print_commands = false; // Adds set -x option (not available for windows)
    let (code, output, error) = run_script::run(
        &format!(r#" {} "#, &init_commands).to_string(),
        &vec![],
        &options
    ).unwrap();
    // @TODO pending replace by impl
    result_commit!(error, code, output);
}

pub fn run(path_object: &Path) -> Result<()> {
    init_repo(&path_object);
    process_dirs(&path_object, &manage_dirs)
}

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

struct Commit {
    pub relative_path_file: String,
    pub path_project: String,
    pub message: String,
    pub date_commit: String,
}

struct ResultCommand {
    pub code: i32,
    pub output: String,
    pub error: String,
}

trait Script {

    fn new(relative_path_file: &String, path_project: &String, date_commit: &String) -> Self; 
    fn run_git_command(&self) -> Result<()>;
    fn result(&self, code: i32,output: String, error: String) -> ResultCommand;
    fn relative_path_file(&self) -> &String;
    fn path_project(&self) -> &String;
    fn message(&self) -> &String;
    fn date_commit(&self) -> &String;
}

impl Commit {
    fn run_init_repo(path_object: &Path) {
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
        result_commit!(error, code, output);
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
}

impl Script for Commit {
    fn new(relative_path_file: &String, path_project: &String, date_commit: &String) -> Commit {
        Commit {
            relative_path_file: relative_path_file.to_string(),
            path_project: path_project.to_string(),
            message: String::from(format!("refactor: add file {:?}", relative_path_file)),
            date_commit: date_commit.to_string(),
        }
    }

    fn relative_path_file(&self) -> &String {
        &self.relative_path_file
    }
    fn path_project(&self) -> &String {
        &self.path_project
    }
    fn message(&self) -> &String {
        &self.message
    }
    fn date_commit(&self) -> &String {
        &self.date_commit
    }

    fn result(&self, code: i32,output: String, error: String) -> ResultCommand {
        ResultCommand {
            code: code, 
            output: output,
            error: error
        }
    }

    fn run_git_command(&self) -> Result<()> {
        let mut options = ScriptOptions::new();
        let big_commands = "cd ".to_owned()+&format!("{}{}{}",&QUOTATION_MARK, &self.path_project(), &QUOTATION_MARK).to_string()+&" && ( git add ".to_owned()+&format!("{}{}{}",&QUOTATION_MARK, &self.relative_path_file(), &QUOTATION_MARK).to_string()+&" || true ) && ( git commit -m ".to_owned()+&format!("{}{}{}",&QUOTATION_MARK, &self.message(), &QUOTATION_MARK).to_string()+&" || true ) && ( git commit --amend --no-edit --date ".to_owned()+&format!("{}{}{}", &QUOTATION_MARK, &self.date_commit(), &QUOTATION_MARK).to_string()+&" || true )".to_owned();
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
        println!("File: {:?}", &self.relative_path_file());
        self.result(code, output, error);
        Ok({})
    }

}

fn manage_dirs(path_project: &Path, dir: &Path, entry: &DirEntry) {
    if let Ok(metadata) = entry.metadata() {
        let system_time = metadata.modified();
        match system_time {
            Ok(n) => {
                if !((&dir.display().to_string()).contains(".git/")) {
                    let date_time = Commit::system_time_to_date_time(&n);
                    let commit: Commit = Commit::new(&dir.display().to_string(), &path_project.display().to_string(), &date_time.to_string());
                    let exe = commit.run_git_command();
                    match exe {
                        Ok(_n) => {
                            println!("Completed iteration");
                        },
                        Err(_) => panic!("Unknown error in iteration"),
                    }
                } else {
                    println!(".git's file omited")
                }

            },
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
        
    } else {
        println!("Couldn't get metadata for {:?}", entry.path());
    }
}

fn process_dirs(dir: &Path, cb: &Fn(&Path, &Path, &DirEntry)) -> Result<String> {
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
    Ok(String::from("Process finalized"))
}

pub fn run(path_object: &Path) -> Result<String> {
    if path_object.display().to_string().trim() == "" {
       panic!("The path that you sent is empty")
    }
    Commit::run_init_repo(&path_object);
    process_dirs(&path_object, &manage_dirs)
}

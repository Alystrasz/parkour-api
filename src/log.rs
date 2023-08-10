use chrono::Local;

pub fn info(msg: &str) {
    print_message(msg, "info");
}

pub fn warn(msg: &str) {
    print_message(msg, "warn");
}

pub fn error(msg: &str) {
    print_message(msg, "error");
}

fn print_message(msg: &str, level: &str) {
    let date = Local::now();
    println!("{}[{}] {}", date.format("[%Y-%m-%d %H:%M:%S]"), level, msg);
}
use colored::Colorize;

pub fn state(topic: &str, message: &str) {
    println!("{} {}", format!(" {topic} ").green().bold(), message);
}

pub fn success(topic: &str, message: &str) {
    println!("{} {}", format!(" {topic} ").green().bold().on_green(), message);
}

pub fn error(topic: &str, message: &str) {
    println!("{} {}", topic.red().bold().on_red(), message);
}

use crate::CONFIG;
use owo_colors::OwoColorize;

#[derive(PartialEq)]
pub enum LogType {
    Error,
    Warning,
    Info,
    Plain
}

pub fn log(text: &str, log_type: LogType) {
    let config = CONFIG.get().expect(&format!("⛔  {}", "Config not initialized!".red()));

    if log_type == LogType::Error { 
        println!("⛔  {}", text.red())
    }

    if config.log {
        match log_type {
            LogType::Warning => { println!("⚠️  {}", text.yellow()) },
            LogType::Info=> { println!("ℹ️  {}", text.bright_blue()) },
            LogType::Plain => { println!("{}", text) },
            _ => ()
        }
    }
}
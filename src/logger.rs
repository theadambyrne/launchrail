use chrono::Local;
use colored::*;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
pub fn init_logger() {
    Builder::new()
        .format(|buf, record| {
            let level = match record.level() {
                log::Level::Error => "ERROR".red(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Info => "INFO".green(),
                log::Level::Debug => "DEBUG".blue(),
                log::Level::Trace => "TRACE".white(),
            };
            writeln!(
                buf,
                "{} [{}] {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                level,
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .write_style(env_logger::WriteStyle::Auto)
        .init();
}

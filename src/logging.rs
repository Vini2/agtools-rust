use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use std::fs::OpenOptions;

pub fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {

    // Configure colored logging levels
    // This will set the colors for different log levels
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::Magenta);

    // Create or open the log file
    let log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("agtools.log")?;

    // File logger
    let file_dispatch = fern::Dispatch::new()
        .level(LevelFilter::Debug)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(log_file);

    // Console logger
    let console_dispatch = fern::Dispatch::new()
        .level(LevelFilter::Info)
        .filter(|metadata| {
            matches!(metadata.level(), log::Level::Info | log::Level::Warn | log::Level::Error)
        })
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                message
            ))
        })
        .chain(std::io::stdout());

    // Global logger
    fern::Dispatch::new()
        .chain(file_dispatch)
        .chain(console_dispatch)
        .apply()?;

    Ok(())
}

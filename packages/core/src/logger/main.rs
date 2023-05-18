use super::color;
use super::color::{bg, font};
use log::{Level, LevelFilter, Metadata, Record};

struct Logger;

static LOGGER: Logger = Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        let meta: &Metadata = record.metadata();
        if !self.enabled(meta) {
            return;
        }
        match meta.level() {
            Level::Trace => println!(
                "{} {} {} - {}",
                font::BRIGHT_BLACK,
                record.level(),
                color::RESET,
                record.args()
            ),
            Level::Debug => println!(
                "{}{} {} {} - {}",
                font::WHITE,
                bg::BRIGHT_BLACK,
                record.level(),
                color::RESET,
                record.args()
            ),
            Level::Info => println!(
                "{}{} {} {} - {}",
                font::WHITE,
                bg::BLUE,
                record.level(),
                color::RESET,
                record.args()
            ),
            Level::Warn => println!(
                "{}{} {} {} - {}",
                font::WHITE,
                bg::YELLOW,
                record.level(),
                color::RESET,
                record.args()
            ),
            Level::Error => eprintln!(
                "{}{} {} {} - {}",
                font::WHITE,
                bg::RED,
                record.level(),
                color::RESET,
                record.args()
            ),
        }
    }

    fn flush(&self) {}
}

pub fn init(filter: LevelFilter) -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER).map(|_| log::set_max_level(filter))
}

pub trait Logger {
    /// Помещает в лог сообщения заданного уровня.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

// TODO: Добавьте определение и реализацию Filter.
struct Filter<T: Logger> {
    logger: T,
    callback: fn(u8, &str) -> bool,
}

impl<T: Logger> Logger for Filter<T> {
    fn log(&self, verbosity: u8, message: &str) {
        if (self.callback)(verbosity, message) {
            self.logger.log(verbosity, message);
        }
    }
}

impl<T: Logger> Filter<T> {
    fn new(logger: T, callback: fn(u8, &str) -> bool) -> Filter<T> {
        return Filter { logger, callback };
    }
}

fn main() {
    let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
    logger.log(5, "FYI");
    logger.log(1, "yikes, something went wrong");
    logger.log(2, "uhoh");
}

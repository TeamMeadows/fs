// global Logger instance

use std::cell::OnceCell;
use atomicframework_api::{Logger, LoggerLevel};

thread_local! {
  static LOGGER: OnceCell<Logger> = const { OnceCell::new() };
}

pub(crate) fn set_logger_instance(logger: Logger) {
  LOGGER.with(|lock| lock.set(logger));
}

#[inline(always)]
pub fn log<T: AsRef<str>>(message: T) {
  log_with_level(LoggerLevel::Debug, message);
}

#[inline(always)]
pub fn log_err<T: AsRef<str>>(message: T) {
  log_with_level(LoggerLevel::Err, message);
}

pub fn log_with_level<T: AsRef<str>>(level: LoggerLevel, message: T) {
  let message = message.as_ref();

  LOGGER.with(|logger| {
    if let Some(logger) = logger.get() {
      logger.log(level, message);
    } else {
      println!("[groom-engine-api][uninitialized] {message}");
    }
  });
}
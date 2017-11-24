use std::env;

use log::{LogRecord, LogLevelFilter};
use env_logger::LogBuilder;

pub fn setup_log() {
    let format = |record: &LogRecord| {
        format!("{level}:  {args}",
        level = record.level(),
        args  = record.args())
    };

    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, LogLevelFilter::Info);

    let log_var ="PEER_LOG";
    if env::var(log_var).is_ok() {
       builder.parse(&env::var(log_var).unwrap());
    }

    builder.init().unwrap();
}
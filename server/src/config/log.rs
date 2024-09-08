use std::{env, fs::OpenOptions};

use chrono::Local;
use tracing_subscriber::fmt::{writer::BoxMakeWriter, Layer};
use tracing_subscriber::{prelude::*, Registry};

use super::env::Env;

pub fn init_logging() {
    let log_level = Env::rust_log();
    env::set_var("RUST_LOG", log_level);

    let date = Local::now().format("%Y-%m-%d").to_string();
    let file_name = format!("logs/{}.log", date);
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)
        .unwrap();
    let file_writer = BoxMakeWriter::new(file);

    let file_layer = Layer::default().with_writer(file_writer);

    Registry::default()
        .with(tracing_subscriber::fmt::Layer::default())
        .with(file_layer)
        .init();
}

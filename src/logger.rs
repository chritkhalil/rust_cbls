use log::{debug, error, info, warn, LevelFilter};

pub fn init() {
    //let _ = env_logger::builder().is_test(true).try_init();
    //env_logger::init();

    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(Some("logger_example"), LevelFilter::Debug)
        .init();
}

pub fn debug(msg: &str) {
    debug!("{}", msg);
}

pub fn info(msg: &str) {
    info!("{}", msg);
}

pub fn warn(msg: &str) {
    warn!("{}", msg);
}

pub fn error(msg: &str) {
    error!("{}", msg);
}

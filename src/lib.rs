#[macro_use]
mod macros;

pub mod address;
pub mod builder;
pub mod device;
pub mod cdevice;
pub mod result_codes;
pub mod socket;

use log::debug;
use log::{LevelFilter, Metadata, Record};

struct SimpleLogger;

impl log::Log for SimpleLogger {
	fn enabled(&self, _metadata: &Metadata) -> bool {
		true
	}

	fn log(&self, record: &Record) {
		if self.enabled(record.metadata()) {
			eprintln!("sio_smoltcp: {} - {}", record.level(), record.args());
		}
	}

	fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

#[no_mangle]
pub extern "C" fn initLogging() {
	eprintln!("sio_smoltcp: Trying to setup logging...");
	log::set_logger(&LOGGER).unwrap();
	log::set_max_level(LevelFilter::Trace);
	debug!("logging set up");
}

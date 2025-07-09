use dwldutil::indicator::{IndicateSignal, Indicator, IndicatorFactory};
use tracing::*;

#[derive(Default)]
pub struct LogIndicator;
impl IndicatorFactory for LogIndicator {
    fn create_task(&self, name: &str, size: u64) -> impl Indicator {
        debug!("Starting download of {} with {}b", name, size);
        let size = if size == 0 { 100 } else { size };
        LogIndicatorChild {
            file: name.to_string(),
            total: size,
        }
    }
}

pub struct LogIndicatorChild {
    file: String,
    total: u64,
}
impl Indicator for LogIndicatorChild {
    fn effect(&mut self, position: u64) {
        let perc = self.total / 100;
        let by = position / perc;
        debug!("FILE: {} => {}/{}b", self.file, by, self.total);
    }
    fn signal(&mut self, signal: dwldutil::indicator::IndicateSignal) {
        match signal {
            IndicateSignal::Fail(f) => {
                error!("ERROR DOWNLOADING FILE {} -- {}", self.file, f);
            }
            IndicateSignal::State(f) => {
                debug!("CHANGING STATE OF FILE {} -> {}", self.file, f);
            }
            IndicateSignal::Success() => {
                debug!("FILE {} DOWNLOAD SUCCESS", self.file);
            }
            IndicateSignal::Start() => {}
        }
    }
}

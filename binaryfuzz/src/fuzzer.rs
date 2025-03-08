use libafl::{monitors::TuiMonitor, Error};
use libafl_bolts::shmem::{ShMemProvider, StdShMemProvider};

pub struct Fuzzer {}

impl Fuzzer {
    pub fn new() -> Fuzzer {
        Fuzzer {}
    }

    pub fn fuzz(&self) -> Result<(), Error> {
        let monitor = TuiMonitor::builder()
            .title("LibAFL QEMU")
            .enhanced_graphics(true)
            .build();
        self.launch(monitor)?;
        Ok(())
    }

    fn launch<M>(&self, monitor: M) -> Result<(), Error> {
        let mut shem_provider = StdShMemProvider::new()?;

        Ok(())
    }
}

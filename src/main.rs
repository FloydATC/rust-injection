


struct RealLogger {
}

impl RealLogger {

    fn new() -> Self {
        Self {
        }
    }
    
    fn log(&self, msg: &str) {
        println!("REAL: {}", msg);
    }
    
}



struct FakeLogger {
}

impl FakeLogger {

    fn new() -> Self {
        Self {
        }
    }
    
    fn log(&self, msg: &str) {
        println!("FAKE: {}", msg);
    }
    
}










fn main() {

    let logger = FakeLogger::new();
    logger.log("Hello world!");

}



//----------
trait IsLogger {
    fn new() -> Self;
    fn log(&self, msg: &str);
}


//---------------
struct RealLogger {
}

impl IsLogger for RealLogger {

    fn new() -> Self {
        Self {
        }
    }
    
    fn log(&self, msg: &str) {
        println!("REAL: {}", msg);
    }
    
}


//---------------
struct FakeLogger {
}

impl IsLogger for FakeLogger {

    fn new() -> Self {
        Self {
        }
    }
    
    fn log(&self, msg: &str) {
        println!("FAKE: {}", msg);
    }
    
}


//--------------
struct AnyLogger<L: IsLogger> {
    logger: L,
}

impl<L: IsLogger> AnyLogger<L> {

    fn new() -> Self {
        Self {
            logger: L::new(),
        }
    }

    fn log(&self, msg: &str) {
        self.logger.log(msg);
    }
}


fn main() {

    let logger = AnyLogger::<FakeLogger>::new();
    logger.log("Hello world!");

}




//----------
trait IsLogger {
    fn new() -> Self;
    fn log(&self, msg: &str);
}


//---------------
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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


//-----------
type Logger = AnyLogger<RealLogger>;


fn main() {

    let logger = Logger::new();
    logger.log("Hello world!");
    
    println!("logger is {:?}", logger);

}


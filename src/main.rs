
use injection::{Logger, AnyLogger, FakeLogger};


fn main() {

    let logger = Logger::new();
    logger.log("Hello world!");
    
    println!("logger is {:?}", logger);

    let fake = AnyLogger::<FakeLogger>::new();
    fake.log("This is a test");

    let custom = AnyLogger::<CustomLogger>::new();
    custom.log("I make dis");

}


struct CustomLogger {
}

impl injection::logger::IsLogger for CustomLogger {

    fn new() -> Self { 
        Self {} 
    }

    fn log(&self, msg: &str) { 
        println!("CUSTOM: {}", msg);
    }
}

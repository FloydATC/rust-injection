
use injection::{Logger, AnyLogger, FakeLogger};


fn main() {

    let logger = Logger::new();
    logger.log("Hello world!");
    
    println!("logger is {:?}", logger);

    let fake = AnyLogger::<FakeLogger>::new();
    fake.log("This is a test");

}


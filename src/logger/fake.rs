

#[derive(Debug)]
pub struct FakeLogger {
}


impl super::IsLogger for FakeLogger {

    fn new() -> Self {
        Self {
        }
    }
    
    fn log(&self, msg: &str) {
        println!("FAKE: {}", msg);
    }
    
}

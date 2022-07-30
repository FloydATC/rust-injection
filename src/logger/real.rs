

#[derive(Debug)]
pub struct RealLogger {
}


impl super::IsLogger for RealLogger {

    fn new() -> Self {
        Self {
        }
    }
    
    fn log(&self, msg: &str) {
        println!("REAL: {}", msg);
    }
    
}


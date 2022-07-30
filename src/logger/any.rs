

pub trait IsLogger {
    fn new() -> Self;
    fn log(&self, msg: &str);
}


pub type Logger = AnyLogger<super::RealLogger>;


#[derive(Debug)]
pub struct AnyLogger<L: IsLogger> {
    logger: L,
}


impl<L: IsLogger> AnyLogger<L> {

    pub fn new() -> Self {
        Self {
            logger: L::new(),
        }
    }

    pub fn log(&self, msg: &str) {
        self.logger.log(msg);
    }
}



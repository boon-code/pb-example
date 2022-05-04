use log::Log;
use indicatif::ProgressBar;

struct MyLog<T: Log> {
    logger: T,
}
impl<T: Log> MyLog<T> {
    pub fn new(logger: T) -> Self {
        MyLog {
            logger
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple() {
        assert_eq!(1,1);
    }
}

use std::{sync::Mutex, collections::HashMap};
use indicatif::ProgressBar;
use once_cell::sync::Lazy;

static GLOBAL_WRAPPER: Lazy<Mutex<MyLogger>> = Lazy::new(|| {
    Mutex::new(MyLogger::new())
});

struct MyLogger {
    val: u32,
}

impl MyLogger {
    pub fn new() -> Self {
        MyLogger { val: 5_u32 }
    }
}

trait PrintImpl {
    fn writeln(&self, text: &str);
}

impl PrintImpl for ProgressBar {
    fn writeln(&self, text: &str) {
        self.println(text);
        // nop
    }
}

struct LogRedir {
}

impl LogRedir {
    pub fn new() -> Self {
        Self { }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{error::Error, thread, time::Duration};

    #[test]
    fn testit() {
        let num = 50_u64;
        let x = LogRedir::new();
        let bar = ProgressBar::new(num);
        for i in 0..num {
            bar.inc(1);
            thread::sleep(Duration::from_millis(100));
            if i % 10 == 0 {
                let msg = format!("Status: {}", i);
                bar.writeln(&msg);
                bar.println(msg);
            }
        }
        bar.finish();
        assert_eq!(1, 1);
    }
}

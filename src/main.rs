use indicatif::ProgressBar;
use std::{error::Error, thread, time::Duration};
type MyResult<T> = Result<T, Box<dyn Error>>;

mod mylog;
mod logwrap;

trait Bla {
    fn inc_one(&self);
}

impl Bla for ProgressBar {
    fn inc_one(&self) {
        self.inc(1);
    }
}

fn main() -> MyResult<()> {
    let num = 100_u64;
    println!("Hello, world!");

    let bar = ProgressBar::new(num);
    for i in 0..num {
        //bar.inc(1);
        bar.inc_one();
        thread::sleep(Duration::from_millis(100));
        if i % 10 == 0 {
            bar.println(format!("Status: {}", i));
        }
    }
    bar.finish();

    Ok(())
}

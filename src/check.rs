use owo_colors::OwoColorize;
use std::fmt::Display;
use std::panic;
use std::time::Duration;

#[macro_export]
macro_rules! check {
    ($fn:expr, $title:expr, $ans:expr) => {
        let start = Instant::now();
        let x = std::panic::catch_unwind(|| $fn);
        let duration = start.elapsed();
        check::show_check_result(stringify!($fn), $title, x, $ans, duration);
    };
}

pub fn show_check_result<T: PartialEq + Display>(
    expr: &str,
    title: &str,
    result: std::thread::Result<T>,
    ans: T,
    time_taken: Duration,
) {
    match result {
        Ok(x) => {
            if x == ans {
                let millis = time_taken.as_millis();
                let timing = if millis == 0 {
                    String::new()
                } else {
                    let mut text = format!(" {}ms", millis);
                    if millis < 1000 {
                        text = format!("{}", text.yellow());
                    } else {
                        text = format!("{}", text.red());
                    }
                    text
                };
                println!("{} {} {}{}", "🗸".green(), expr, title, timing);
            } else {
                println!(
                    "❌{} = {} (should be {})",
                    expr,
                    x.to_string().red(),
                    ans.to_string().green()
                );
            }
        }
        Err(e) => {
            println!("❌{}: {}", expr, "panic!".red());
            panic::resume_unwind(e)
        }
    }
}

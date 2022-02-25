use owo_colors::OwoColorize;
use std::time::Duration;

#[macro_export]
macro_rules! check {
    ($fn:expr, $title:expr, $ans:expr) => {
        let start = Instant::now();
        let x: i64 = $fn;
        let duration = start.elapsed();
        check::show_check_result(stringify!($fn), $title, x, $ans, duration);
    };
}

pub fn show_check_result(expr: &str, title: &str, x: i64, ans: i64, time_taken: Duration) {
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

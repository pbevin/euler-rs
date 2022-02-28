mod best;
mod decimal;
mod factors;
mod fibs;
mod partitions;

use linkme::distributed_slice;
use owo_colors::OwoColorize;
use std::time::Duration;

pub use best::*;
pub use decimal::*;
pub use factors::*;
pub use fibs::*;
pub use partitions::*;

#[distributed_slice]
pub static SOLUTIONS: [fn() -> EulerResult] = [..];

#[macro_export]
macro_rules! solution {
    ($fn:ident, $title:expr, $expected:expr) => {
        paste::paste! {
            #[linkme::distributed_slice(euler::SOLUTIONS)]
            fn [<check_ $fn>]() -> euler::EulerResult {
                let start = std::time::Instant::now();
                let result = std::panic::catch_unwind(|| $fn()).ok();
                let time_taken = start.elapsed();

                let (expected, actual) = euler::match_answers(Some($expected), result);

                euler::EulerResult {
                    name: stringify!($fn).to_string(),
                    title: $title.to_string(),
                    expected,
                    actual,
                    time_taken,
                }
            }
        }
    };
}

pub fn match_answers<T: std::fmt::Display>(
    expected: Option<T>,
    actual: Option<T>,
) -> (Option<String>, Option<String>) {
    (
        expected.map(|e| e.to_string()),
        actual.map(|a| a.to_string()),
    )
}

pub struct EulerResult {
    pub name: String,
    pub title: String,
    pub expected: Option<String>,
    pub actual: Option<String>,
    pub time_taken: Duration,
}

impl EulerResult {
    pub fn is_correct(&self) -> bool {
        matches!(self.rating(), Rating::Correct(_))
    }

    pub fn rating(&self) -> Rating {
        match (&self.actual, &self.expected) {
            (None, _) => Rating::Panic,
            (Some(actual), None) => Rating::Uncheckable(actual),
            (Some(actual), Some(expected)) => {
                if actual == expected {
                    Rating::Correct(actual)
                } else {
                    Rating::Wrong { actual, expected }
                }
            }
        }
    }

    fn timing(&self) -> String {
        let millis = self.time_taken.as_millis();
        if millis == 0 {
            String::new()
        } else {
            let mut text = format!(" {}ms", millis);
            if millis < 1000 {
                text = format!("{}", text.yellow());
            } else {
                text = format!("{}", text.red());
            }
            text
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rating<'a> {
    Panic,
    Uncheckable(&'a str),
    Correct(&'a str),
    Wrong { actual: &'a str, expected: &'a str },
}

pub fn check_all_solutions() {
    let mut all_correct = true;
    for fun in SOLUTIONS {
        let result = fun();
        show_result(&result);
        if !result.is_correct() {
            all_correct = false;
        }
    }

    println!("-----------------");
    if all_correct {
        println!("{} All good", "🗸".green());
    } else {
        println!("❌ Errors");
    }
}

fn show_result(result: &EulerResult) {
    match result.rating() {
        Rating::Panic => {
            println!("❌{}: {}", result.name, "panic!".red());
        }
        Rating::Uncheckable(answer) => {
            println!(
                "{}: {} {}{}",
                result.name,
                result.title.yellow(),
                answer,
                result.timing()
            )
        }
        Rating::Correct(_) => {
            println!(
                "{} {} {}{}",
                "🗸".green(),
                result.name,
                result.title,
                result.timing()
            );
        }
        Rating::Wrong { actual, expected } => {
            println!(
                "❌{}: got {},\n\
                    should be {}",
                result.name,
                actual.red(),
                expected.green()
            );
        }
    }
}

use owo_colors::OwoColorize;
use rayon::prelude::*;
use std::time::Duration;
use std::time::Instant;

pub fn check_all_solutions() {
    let all_correct = super::SOLUTIONS
        .par_iter()
        .map(|func| func())
        .map(|runner| {
            let (rating, timing) = runner.run();
            (runner, rating, timing)
        })
        .inspect(|(runner, rating, timing)| {
            show_result(runner, rating, timing);
        })
        .all(|(_, rating, _)| rating == Rating::CorrectAnswer);

    println!("-----------------");
    if all_correct {
        println!("{} All good", "🗸".green());
    } else {
        println!("❌ Errors");
    }
}

fn show_result(runner: &TestRunner, rating: &Rating, time_taken: &Duration) {
    match rating {
        Rating::Panicked => {
            println!("❌{}: {}", runner.name, "panic!".red());
        }
        Rating::Answered(answer) => {
            println!(
                "{}: {} {}{}",
                runner.name,
                runner.title.yellow(),
                answer,
                format_timing(time_taken)
            )
        }
        Rating::CorrectAnswer => {
            println!(
                "{} {} {}{}",
                "🗸".green(),
                runner.name,
                runner.title,
                format_timing(time_taken)
            );
        }
        Rating::WrongAnswer { actual, expected } => {
            println!(
                "❌{}: got {},\n\
                    should be {}",
                runner.name,
                actual.red(),
                expected.green()
            );
        }
    }
}

pub fn check_answer<T: std::fmt::Display + PartialEq>(
    actual: Option<T>,
    expected: &Option<T>,
) -> Rating {
    match (&actual, &expected) {
        (None, _) => Rating::Panicked,
        (Some(a), None) => Rating::Answered(a.to_string()),
        (Some(a), Some(e)) => {
            if a == e {
                Rating::CorrectAnswer
            } else {
                Rating::WrongAnswer {
                    actual: a.to_string(),
                    expected: e.to_string(),
                }
            }
        }
    }
}

pub struct TestRunner {
    name: String,
    title: String,
    checkable: Box<dyn Checkable + Send>,
}

impl TestRunner {
    pub fn run(&self) -> (Rating, Duration) {
        let start_time = Instant::now();
        let rating = self.checkable.check();
        let duration = start_time.elapsed();
        (rating, duration)
    }
}

impl TestRunner {
    pub fn new<T: 'static + std::fmt::Display + PartialEq + Send>(
        name: impl Into<String>,
        title: impl Into<String>,
        func: Box<dyn Fn() -> Option<T> + Send>,
        expected: T,
    ) -> TestRunner {
        Self {
            name: name.into(),
            title: title.into(),
            checkable: Box::new((func, Some(expected))),
        }
    }
}

pub trait Checkable {
    fn check(&self) -> Rating;

    fn is_correct(&self) -> bool {
        matches!(self.check(), Rating::CorrectAnswer)
    }
}

impl<T: std::fmt::Display + PartialEq> Checkable
    for (Box<dyn Fn() -> Option<T> + Send>, Option<T>)
{
    fn check(&self) -> Rating {
        check_answer(self.0(), &self.1)
    }
}

fn format_timing(duration: &Duration) -> String {
    let millis = duration.as_millis();
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

#[derive(Debug, Clone, PartialEq)]
pub enum Rating {
    Panicked,
    Answered(String),
    CorrectAnswer,
    WrongAnswer { actual: String, expected: String },
}

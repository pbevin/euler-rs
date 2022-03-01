mod best;
mod decimal;
mod factors;
mod fibs;
mod partitions;
mod runner;

pub use best::Best;
pub use best::CountOf;

pub use decimal::is_palindrome;
pub use decimal::POWERS_OF_10;

pub use factors::factors;

pub use fibs::fibs;
pub use fibs::Fibs;

pub use partitions::partitions3;

pub use runner::check_all_solutions;
pub use runner::Checkable;
pub use runner::TestRunner;

use linkme::distributed_slice;

pub type TestParams = (&'static str, &'static str, Box<dyn Checkable + Sync>);

/// Contains all the project's test runners.
#[distributed_slice]
pub static SOLUTIONS: [fn() -> TestRunner] = [..];

#[macro_export]
macro_rules! solution {
    ($fn:ident, $title:expr, $expected:expr) => {
        paste::paste! {
            #[linkme::distributed_slice(euler::SOLUTIONS)]
            fn [<runner_ $fn>]() -> euler::TestRunner {
                let func = || { std::panic::catch_unwind(|| $fn()).ok() };
                euler::TestRunner::new(stringify!($fn), $title, Box::new(func), $expected)
            }
        }
    };
}

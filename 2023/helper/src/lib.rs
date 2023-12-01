/// A macro to check if the answer is correct and print it.
/// While solving a problem, use the `println!` macro to print the answer.
/// When you are done, replace the `println!` macro with this macro and add the expected answer as the third argument.
///
/// Replace this:
/// ```ignore
/// println!("Day 1 part 1: {}", add(1, 2));
/// ```
/// With this:
/// ```ignore
/// solved!("Day 1 part 1: {}", add(1, 2), 3);
/// ```
#[macro_export]
macro_rules! solved {
    ($message:literal, $answer:expr, $expect:literal) => {
        assert!($expect == $answer, "Got {}, expected {}", $answer, $expect);
        println!($message, $answer);
    };
}

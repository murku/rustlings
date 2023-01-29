// options1.rs
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a hint.

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22
    // The Option output should gracefully handle cases where time_of_day > 24.
    // TODO: Complete the function body - remember to return an Option!
    let result: Option<u16>;
    match time_of_day {
        0..=21 => result = Some(5),
        22..=23 => result = Some(0),
        other => result = None,
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(21), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(24), None);
        assert_eq!(maybe_icecream(25), None);
        assert_eq!(maybe_icecream(65535), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the Option?
        let input = 12;
        // let input: u16 = 12;
        // let input: u16 = 25;
        let expected = 5;
        // let expected: u16 = 5;
        
        if let Some(i) = maybe_icecream(input) {
            assert_eq!(i, expected);
        } else {
            assert!(true)
        }
    }
}

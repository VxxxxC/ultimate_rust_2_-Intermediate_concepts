// 1. This code looks terrible. Let's start cleaning this up by running `cargo fmt`. If you
// configured your editor or IDE to run `cargo fmt` automatically upon save, you can just save!

// 2. `cargo fmt` is great, but it doesn't add blank lines where there are none. Go ahead and add
// some blank lines in places you think it would make sense.

// 3. Time to clean up! Run `cargo clippy`. Fix up all the warnings so `cargo clippy` is silent.

// Challenge: Clippy doesn't find *everything*. What else would you change to make this code better?

//! # Hi! This is inner doc, on top of outter doc [///]
//! below is PI and count_to_5
/// THIS IS CONSTANT VALUE OF PI
/// # PI
///
/// It could generate by :
/// - `std::f32:consts::PI`
/// - [`PI`] [PI]
pub const PI: f32 = std::f32::consts::PI;

/**
* THIS IS COUNT_TO_5 FN
* # count_to_5
**/
pub fn count_to_5() -> i32 {
    let mut f_oo = 0;
    loop {
        if f_oo > PI as i32 && f_oo >= 5 {
            break;
        }
        f_oo += 1;
        print!("f_oo : {}", &f_oo)
    }
    f_oo
}
pub fn main() {
    println!("I can count to {}", count_to_5());
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting() {
        assert!(count_to_5() == 5, "count : {}", count_to_5());
    }
}

fn round_to_four_decimal_places(x: f64) -> f64 {
    (x * 10000.0).round() / 10000.0
}

pub fn new_birthday_probability(n: u32) -> f64 {
    if n > 365 {
        round_to_four_decimal_places(1.0)
    } else {
        let mut diff = 1.0;
        for i in (365 - n + 1)..=365 {
            diff *= i as  f64 / 365.0;
        }
        round_to_four_decimal_places(1.0 - diff)    
    }
}

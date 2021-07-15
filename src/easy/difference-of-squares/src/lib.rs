pub fn square_of_sum(n: u32) -> u32 {
    (1..=n).fold(0, |a, b| a + b).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|x| x.pow(2))
           .fold(0, |a, b| a + b)
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

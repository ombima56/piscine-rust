pub fn first_fifty_even_square() -> Vec<i32> {
    let square = |x: i32| x * x;

    (0..).filter(|x| x % 2 == 0)
        .take(50)
        .map(square)
        .collect()
}

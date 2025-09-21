fn is_number(input: &str) -> bool {
    input.trim().parse::<i32>().is_ok()
}

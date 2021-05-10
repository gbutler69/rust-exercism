pub fn raindrops(n: u32) -> String {
    let mut result = String::new();
    if n % 3 == 0 {
        result = "Pling".into();
    }
    if n % 5 == 0 {
        result += "Plang".into();
    }
    if n % 7 == 0 {
        result += "Plong".into();
    }
    if result.is_empty() {
        result = format!("{}", n)
    }
    result
}

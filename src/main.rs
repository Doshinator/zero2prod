fn main() {
    println!("Hello, world!");
}

pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 4);
        assert_eq!(result, 6);
    }
}

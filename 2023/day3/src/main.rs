fn main() {
    let input = include_str!("input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let sample: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let sum: i32 = part1(&sample);
        assert_eq!(sum, 142);
    }
}
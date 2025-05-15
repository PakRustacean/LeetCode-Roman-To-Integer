struct Solution;

impl Solution {
    pub fn roman_to_int(s: &String) -> i32 {
        let roman_values = |c: char| -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        };

        let chars: Vec<char> = s.chars().collect();
        let mut total = 0;
        let mut i = 0;

        while i < chars.len() {
            let current = roman_values(chars[i]);
            if i + 1 < chars.len() {
                let next = roman_values(chars[i + 1]);
                if current < next {
                    total += next - current;
                    i += 2;
                    continue;
                }
            }
            total += current;
            i += 1;
        }
        total
    }
}

fn main() {
    let input = "MCMXCIV".to_string();
    let result = Solution::roman_to_int(&input);
    println!("{} = {}", input, result)
}

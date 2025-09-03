pub fn is_armstrong_number(num: u32) -> bool {
        let digits: Vec<u32> = {
            let mut n = num;
            let mut v = Vec::new();
            if n == 0 {
                v.push(0);
            } else {
                while n > 0 {
                    v.push(n % 10);
                    n /= 10;
                }
                v.reverse();
            }
            v
        };
        let power = digits.len() as u32;
        let sum: u32 = digits.iter()
            .map(|&d| d.pow(power))
            .sum();
        sum == num
}

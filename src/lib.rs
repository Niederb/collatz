/// Calculate the collatz steps for the specified number
pub fn get_steps(number: u128) -> Vec<u128> {
    let mut next = number;
    let mut steps = vec![];
    loop {
        next = if next % 2 == 0 {
            next / 2
        } else {
            3 * next + 1
        };
        steps.push(next);
        if next == 1 {
            break;
        }
    }
    steps
}

/// Count the number of collatz steps for the specified number
pub fn count_steps(number: u128) -> u128 {
    let mut next = number;
    let mut steps = 0;
    loop {
        next = if next % 2 == 0 {
            steps += 1;
            next / 2
        } else {
            steps += 2;
            (3 * next + 1) / 2
        };
        if next == 1 {
            break;
        }
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_steps() {
        let numbers = [
            9,
            97,
            871,
            6171,
            77_031,
            837_799,
            8_400_511,
            931_386_509_544_713_451,
        ];
        let steps = [19, 118, 178, 261, 350, 524, 685, 2283];
        for (i, n) in numbers.iter().enumerate() {
            assert_eq!(steps[i], count_steps(*n));
            assert_eq!(steps[i], get_steps(*n).len() as u128);
        }
    }

    #[test]
    fn test_get_steps() {
        let steps = vec![
            58, 29, 88, 44, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1,
        ];
        assert_eq!(steps, get_steps(19));

        let steps = vec![
            82, 41, 124, 62, 31, 94, 47, 142, 71, 214, 107, 322, 161, 484, 242, 121, 364, 182, 91,
            274, 137, 412, 206, 103, 310, 155, 466, 233, 700, 350, 175, 526, 263, 790, 395, 1186,
            593, 1780, 890, 445, 1336, 668, 334, 167, 502, 251, 754, 377, 1132, 566, 283, 850, 425,
            1276, 638, 319, 958, 479, 1438, 719, 2158, 1079, 3238, 1619, 4858, 2429, 7288, 3644,
            1822, 911, 2734, 1367, 4102, 2051, 6154, 3077, 9232, 4616, 2308, 1154, 577, 1732, 866,
            433, 1300, 650, 325, 976, 488, 244, 122, 61, 184, 92, 46, 23, 70, 35, 106, 53, 160, 80,
            40, 20, 10, 5, 16, 8, 4, 2, 1,
        ];
        assert_eq!(steps, get_steps(27));
    }
}

use crate::vecint::VecUInt;

struct FibonacciIter {
    prev2: VecUInt,
    prev1: VecUInt,
}

impl FibonacciIter {
    fn new() -> FibonacciIter {
        FibonacciIter {
            prev2: VecUInt::from(0),
            prev1: VecUInt::from(0),
        }
    }
}

impl Iterator for FibonacciIter {
    type Item = VecUInt;

    fn next(&mut self) -> Option<Self::Item> {
        let result = {
            let sum = self.prev2.clone() + &self.prev1;
            if sum == 0 {
                1.into()
            } else {
                sum
            }
        };
        let new_prev2 = self.prev1.clone();

        *self = FibonacciIter {
            prev2: new_prev2,
            prev1: result.clone(),
        };
        Some(result)
    }
}

fn solve() -> usize {
    const NUMBER_OF_DIGITS_TO_SEARCH_FOR: u32 = 1000;

    let first_number_with_enough_digits =
        VecUInt::from(10).pow((NUMBER_OF_DIGITS_TO_SEARCH_FOR - 1) as usize);
    //println!("Big enough: {}", first_number_with_enough_digits);

    FibonacciIter::new()
        .enumerate()
        //.inspect(|(index, value)| println!("{}: {}", index, value))
        .skip_while(|(_, value)| value < &first_number_with_enough_digits)
        .nth(0)
        .map(|(index, _)| index + 1)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::FibonacciIter;

    #[test]
    fn fibonacci_start() {
        let a = FibonacciIter::new()
            .map(|x| u32::try_from(&x).unwrap())
            .take(12)
            .collect::<Vec<_>>();
        let b = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
        assert_eq!(a, b);
    }
}

pub fn solve_str() -> String {
    format!("{}", solve())
}

fn is_abundant(n: u32) -> bool {
    let proper_divisors = {
        let mut divs: Vec<u32> = vec![1];
        let mut i = 2_u32;
        let mut last: u32 = n / 2 + 1;
        loop {
            if i >= last {
                break;
            }
            let remainder = n % i;
            if remainder == 0 {
                divs.push(i);
                let divisor_other = n / i;
                assert!(n % divisor_other == 0);
                if divisor_other != i {
                    divs.push(divisor_other);
                }
                last = divisor_other;
            }
            i += 1;
        }
        divs
    };

    let divisor_sum = proper_divisors.iter().sum::<u32>();

    divisor_sum > n
}

fn is_sum_of_two_abundant_numbers(number: u32, abundant_numbers: &Vec<u32>) -> bool {
    let mut iter_first = abundant_numbers.iter();
    loop {
        let mut iter_second = iter_first.clone();

        let first = match iter_first.next() {
            None => break,
            Some(x) => *x,
        };
        if first > number / 2 {
            break;
        }

        loop {
            let second = match iter_second.next() {
                None => break,
                Some(x) => *x,
            };

            let sum = first + second;

            if sum == number {
                return true;
            }
            if sum > number {
                break;
            }
        }
    }
    false
}

fn solve() -> u32 {
    let mut i = 1_u32;
    let mut abundant_numbers: Vec<u32> = vec![];
    let mut sum = 0_u32;
    loop {
        if i >= 28123 {
            break;
        }
        //println!("Checking number: {}", i);

        if !is_sum_of_two_abundant_numbers(i, &abundant_numbers) {
            sum += i;
        }

        if is_abundant(i) {
            abundant_numbers.push(i);
        }

        i += 1;
    }

    let first_too_large_summed_number = abundant_numbers.last().unwrap() + 1;
    for j in first_too_large_summed_number..i {
        sum -= j;
    }

    sum
}

pub fn print() {
    let answer = solve();
    println!("Answer: {}", answer);
}

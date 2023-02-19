fn find_recurring_fraction(fraction: &Vec<u8>) -> Option<Vec<u8>> {
    if fraction.len() < 20 {
        // Keep getting more data
        return None;
    }

    const NUMBER_OF_RECURRING_CHUNKS: usize = 4;

    for chunk_len in 1..=(fraction.len() / NUMBER_OF_RECURRING_CHUNKS) {
        let chunks = fraction
            .rchunks(chunk_len)
            .take_while(|x| x.len() == chunk_len)
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<u8>>>();
        let last_chunk: Vec<u8> = chunks.first().unwrap().clone();
        if chunks
            .iter()
            .take(NUMBER_OF_RECURRING_CHUNKS)
            .all(|x| x == &last_chunk)
        {
            return Some(last_chunk);
        }
    }
    None
}

fn get_recurring_fraction(divisor: u32) -> Option<Vec<u8>> {
    let mut fraction = vec![];
    let mut recurring_fraction = None;
    let _state = fraction::division::divide_rem(1, divisor, |state, d| {
        fraction.push(d);
        match find_recurring_fraction(&fraction) {
            None => Ok(Ok(state)),
            Some(x) => {
                recurring_fraction = Some(x);
                Ok(Err(state))
            }
        }
    })
    .unwrap();
    //println!("Value: {:?}", recurring_fraction);
    //println!("State: {:?}", _state);
    recurring_fraction
}

#[cfg(test)]
mod tests {
    use super::get_recurring_fraction;

    #[test]
    fn examples() {
        assert_eq!(get_recurring_fraction(2), None);
        assert_eq!(get_recurring_fraction(3), Some(vec![3]));
        assert_eq!(get_recurring_fraction(4), None);
        assert_eq!(get_recurring_fraction(5), None);
        assert_eq!(get_recurring_fraction(6), Some(vec![6]));
        assert_eq!(get_recurring_fraction(7), Some(vec![1, 4, 2, 8, 5, 7]));
        assert_eq!(get_recurring_fraction(8), None);
        assert_eq!(get_recurring_fraction(9), Some(vec![1]));
        assert_eq!(get_recurring_fraction(10), None);
    }
}

fn solve() -> u32 {
    let mut resulting_divisor = 0;
    let mut longest_recurring_fraction = 0;
    for i in 2..1000 {
        match get_recurring_fraction(i) {
            None => {
                println!("Divisor {}; No recurring fraction", i);
            }
            Some(x) => {
                let len = x.len();
                println!("Divisor {}; Recurring fraction length: {}", i, len);
                if len > longest_recurring_fraction {
                    longest_recurring_fraction = len;
                    resulting_divisor = i;
                    println!("New longest fraction");
                }
            }
        }
    }
    resulting_divisor
}

pub fn solve_str() -> String {
    format!("{}", solve())
}

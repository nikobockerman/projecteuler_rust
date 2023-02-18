use std::collections::BTreeSet;

struct OrderedArrangement {
    digits: Vec<u32>,
    first_returned: bool,
    min: u32,
    max: u32,
}

impl OrderedArrangement {
    fn new(digits: BTreeSet<u32>) -> OrderedArrangement {
        let min = *digits.first().unwrap();
        let max = *digits.last().unwrap();
        assert!(digits.len() == (max - min + 1) as usize);
        OrderedArrangement {
            digits: digits.iter().map(|x| *x).collect(),
            first_returned: false,
            min,
            max,
        }
    }
}

impl Iterator for OrderedArrangement {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = {
                if !self.first_returned {
                    self.first_returned = true;
                    Some(&self.digits)
                } else if self.digits.iter().all(|x| x == &self.max) {
                    None
                } else {
                    let index_to_change = self
                        .digits
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(_, x)| x < &&self.max)
                        .map(|(index, _)| index)
                        .unwrap();
                    self.digits[index_to_change] += 1;
                    self.digits
                        .iter_mut()
                        .skip(index_to_change + 1)
                        .for_each(|x| *x = self.min);
                    Some(&self.digits)
                }
            };
            match next {
                None => return None,
                Some(x) => {
                    if BTreeSet::from_iter(x.iter()).len() == x.len() {
                        return Some(x.clone());
                    } else {
                        continue;
                    }
                }
            }
        }
    }
}

fn solve() -> String {
    let ordered_arrangement =
        OrderedArrangement::new(BTreeSet::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
    let result = ordered_arrangement
        .enumerate()
        .inspect(|(index, x)| println!("{}: {:?}", index, x))
        .map(|(_, x)| x)
        .nth(1_000_000 - 1)
        .unwrap();
    let answer = result.iter().map(|x| x.to_string()).collect::<String>();
    answer
}

pub fn print() {
    let answer = solve();
    println!("Answer: {}", answer);
}

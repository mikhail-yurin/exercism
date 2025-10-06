use std::collections::HashSet;

fn is_palindrome(str: String) -> bool {
    if str.len() == 1 {
        true
    } else {
        let str_rev: String = str.chars().rev().collect();
        let mid = str.len() / 2;

        let left = &str[0..mid];
        let right = &str_rev[0..mid];

        left == right
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    min: u64,
    max: u64,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        let num = self.value;
        let mut result_set: HashSet<(u64, u64)> = HashSet::new();

        let mut divider: u64 = self.min;

        while divider <= self.max && divider * divider <= num {
            if num % divider == 0 {
                let other = num / divider;
                if other >= self.min && other <= self.max {
                    let pair = if divider <= other {
                        (divider, other)
                    } else {
                        (other, divider)
                    };
                    result_set.insert(pair);
                }
            }
            divider += 1;
        }

        result_set
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut result: Option<(Palindrome, Palindrome)> = None;

    for a in min..=max {
        for b in a..=max {
            let num = a * b;

            if is_palindrome(num.to_string()) {
                if result.is_none() {
                    result = Some((
                        Palindrome {
                            value: num,
                            min,
                            max,
                        },
                        Palindrome {
                            value: num,
                            min,
                            max,
                        },
                    ));
                } else if num < result.as_ref().unwrap().0.value() {
                    if let Some(ref mut pair) = result {
                        pair.0 = Palindrome {
                            value: num,
                            min,
                            max,
                        };
                    }
                } else if num > result.as_ref().unwrap().1.value() {
                    if let Some(ref mut pair) = result {
                        pair.1 = Palindrome {
                            value: num,
                            min,
                            max,
                        };
                    }
                }
            }
        }
    }

    result
}

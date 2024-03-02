use std::ops::RangeBounds;

use primefactor::PrimeFactors;

fn two_prime(x: u128) -> Option<Vec<u128>> {
    let prime_factor = PrimeFactors::from(x);

    if prime_factor.len() != 2 {
        return None;
    }

    for int_factor in prime_factor.iter() {
        if int_factor.exponent > 1 {
            return None;
        }
    }

    Some(
        prime_factor
            .iter()
            .map(|int_factor| int_factor.integer)
            .collect(),
    )
}

fn three_prime(x: u128) -> Option<Vec<u128>> {
    let prime_factor = PrimeFactors::from(x);
    if prime_factor.len() != 3 {
        return None;
    }
    Some(
        prime_factor
            .iter()
            .map(|int_factor| int_factor.integer)
            .collect(),
    )
}

fn revert_number(x: u128) -> u128 {
    let mut number = x;
    let mut reverted_number = 0;
    while number > 0 {
        reverted_number = reverted_number * 10 + number % 10;
        number /= 10;
    }
    reverted_number
}

fn is_all_different(pq: &[u128], rst: &[u128]) -> bool {
    let mut all_numbers = pq.iter().chain(rst.iter()).collect::<Vec<_>>();
    let length = all_numbers.len();
    all_numbers.sort();
    all_numbers.dedup();
    all_numbers.len() == length
}

fn find_t_number<R>(range: R) -> Vec<u128>
where
    R: IntoIterator<Item = u128>,
{
    let mut t_number: Vec<u128> = Vec::new();

    for number in range {
        if let Some(pq) = two_prime(number) {
            if let Some(rst) = three_prime(revert_number(number)) {
                if is_all_different(&pq, &rst) {
                    t_number.push(number);
                    println!(
                        "{}: ({}, {}), {}: ({}, {}, {})",
                        number,
                        pq[0],
                        pq[1],
                        revert_number(number),
                        rst[0],
                        rst[1],
                        rst[2]
                    );
                }
            }
        }
    }

    t_number
}

fn main() {
    let t_number = find_t_number(1..1000000);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_two_prime() {
        assert_eq!(is_two_prime(4), false);
        assert_eq!(is_two_prime(6), true);
    }
}

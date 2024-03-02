use std::ops::RangeBounds;

use primefactor::PrimeFactors;

fn is_two_prime(x: u128) -> bool {
    let prime_factor = PrimeFactors::from(x);

    if prime_factor.len() != 2 {
        return false;
    }

    for int_factor in prime_factor.iter() {
        if int_factor.exponent > 1 {
            return false;
        }
    }

    true
}

fn calc_two_prime(x: u128) -> Option<Vec<u128>> {
    let prime_factor = PrimeFactors::from(x);

    if prime_factor.len() != 2 {
        return None;
    }

    Some(
        prime_factor
            .iter()
            .map(|int_factor| int_factor.integer)
            .collect(),
    )
}

fn is_three_prime(x: u128) -> bool {
    let prime_factor = PrimeFactors::from(x);
    if prime_factor.len() != 3 {
        return false;
    }
    for int_factor in prime_factor.iter() {
        if int_factor.exponent > 1 {
            return false;
        }
    }
    true
}

fn calc_three_prime(x: u128) -> Option<Vec<u128>> {
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

fn find_t_number<R>(range: R) -> Vec<u128>
where
    R: IntoIterator<Item = u128>,
{
    let mut t_number: Vec<u128> = Vec::new();

    for number in range {
        if is_two_prime(number) && is_three_prime(revert_number(number)) {
            t_number.push(number);
        }
    }

    t_number
}

fn print_t_number(t_number: Vec<u128>) {
    for number in t_number {
        let pq = calc_two_prime(number).unwrap();
        let rst = calc_three_prime(revert_number(number)).unwrap();

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

fn main() {
    let t_number = find_t_number(1000..10000);

    print_t_number(t_number);
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

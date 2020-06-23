pub const MAXNUM: usize = 10000;

pub fn first_10000_primes() -> u32 {
    let mut primes = vec![2; MAXNUM];
    let mut count = 1;

    let mut flag;
    let mut i = 3;
    loop {
        let mut j = 0;
        loop {
            flag = (primes[j] * primes[j] > i);
            if flag || i % primes[j] == 0 {
                break;
            }
            j += 1;
        }
        if flag {
            primes[count] = i;
            count += 1;
        }
        if count == MAXNUM {
            return primes[MAXNUM - 1];
        }
        i += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(104729, first_10000_primes());
    }
}

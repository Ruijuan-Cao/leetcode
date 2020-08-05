pub const MAXNUM: usize = 10000;

pub fn first_10000_primes() -> u32 {
    let mut primes = vec![2; MAXNUM];
    let mut index = 1;

    let mut i = 3;
    loop {
        let mut flag = false;
        let mut j = 1;
        loop {
            flag = (primes[j] * primes[j] > i);
            if flag || i % primes[j] == 0 {
                break;
            }
            j += 1;
        }
        if flag {
            primes[index] = i;
            index += 1;
        }
        if index == MAXNUM {
            return primes[index - 1];
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

// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    let mut sum: u64 = 1;
    while num > 0 {
        match num {
            1 => 1,
            _ => {
                sum *= num;
                num -= 1;
            }
        }
    }
    sum
}

// NOTE: 实现factorial的方式大全

fn factorial1(n: u64) -> u64 {
    match n {
        0 => 1,
        _ => n * factorial1(n-1),
    }
}

fn factorial2(n: u64) -> u64 {
    fn fact_acc(n: u64, acc: u64) -> u64 {
        match n {
            0 => acc,
            _ => fact_acc(n-1, n * acc),
        }
    }
    fact_acc(n,1)
}

fn factorial3(n: u64) -> u64 {
    let mut sum = 1;
    for i in 2..=n {
        sum *= i;
    }
    result
}

fn factorial4(n: u64) -> u64 {
    (1..=n).fold(1, |acc, x| acc * x)
}

fn factorial5(n: u64) -> u64 {
    (1..=n).product()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}

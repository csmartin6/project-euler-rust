use utils;
use std::collections::{HashMap, VecDeque};


pub fn problem_001() -> usize {
    let mut s: usize = 0;

    for x in 0..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            s += x;
        }
    }
    s
}

pub fn problem_002() -> usize {
    let n = 4000000;

    let mut fib: Vec<usize> = vec![1, 1];

    loop {
        let f_2 = fib[fib.len() - 2];
        let f_1 = fib[fib.len() - 1];
        if f_1 + f_2 > n {
            break;
        }
        fib.push(f_2 + f_1);

    }

    let mut sum = 0;
    for x in fib {
        if x % 2 == 0 {
            sum += x;
        }

    }
    return sum;
}

pub fn problem_003() -> usize {
    let n: usize = 600851475143;
    *utils::prime_factors(n).iter().max().unwrap()
}




pub fn problem_004() -> usize {

    let mut max_palindrome: usize = 0;
    for (x, y) in iproduct!(900..1000, 900..1000) {
        if x * y > max_palindrome && utils::is_palindrome(x * y) {
            max_palindrome = x * y;
        }
    }
    max_palindrome
}

pub fn problem_005() -> usize {
    let n = 21;

    let mut cumulative_prime_factors: HashMap<usize, usize> = HashMap::new();

    for i in 1..n {
        let factors = utils::prime_factors(i);
        let factor_count = utils::counts(&factors[..]);
        for (k, &v) in factor_count.iter() {
            if !cumulative_prime_factors.contains_key(k) ||
               factor_count[k] > cumulative_prime_factors[k] {
                cumulative_prime_factors.insert(*k, v);
            }
        }
    }

    let mut product = 1;

    for (k, &v) in cumulative_prime_factors.iter() {
        product *= k.pow(v as u32);
    }

    product
}

pub fn problem_006() -> usize {
    let n = 100;
    let sum_of_squares: usize = (1..n + 1).map(|x| x * x).fold(0, |sum, x| sum + x);
    let square_of_sum: usize = (1..n + 1).fold(0, |sum, x| sum + x).pow(2);
    square_of_sum - sum_of_squares
}

pub fn problem_007() -> usize {
    let n = 10001;
    let max_candidate = 2 * n * (n as f64).ln() as usize;

    let primes = utils::prime_sieve(max_candidate);
    primes[n - 1]
}


pub fn problem_008() -> u64 {

    let num = "73167176531330624919225119674426574742355349194934
    96983520312774506326239578318016984801869478851843
    85861560789112949495459501737958331952853208805511
    12540698747158523863050715693290963295227443043557
    66896648950445244523161731856403098711121722383113
    62229893423380308135336276614282806444486645238749
    30358907296290491560440772390713810515859307960866
    70172427121883998797908792274921901699720888093776
    65727333001053367881220235421809751254540594752243
    52584907711670556013604839586446706324415722155397
    53697817977846174064955149290862569321978468622482
    83972241375657056057490261407972968652414535100474
    82166370484403199890008895243450658541227588666881
    16427171479924442928230863465674813919123162824586
    17866458359124566529476545682848912883142607690042
    24219022671055626321111109370544217506941658960408
    07198403850962455444362981230987879927244284909188
    84580156166097919133875499200524063689912560717606
    05886116467109405077541002256983155200055935729725
    71636269561882670428252483600823257530420752963450";

    let n = 13;
    let mut max_prod: u64 = 0;

    let mut queue = VecDeque::with_capacity(n);

    for s in num.chars() {
        if s.is_digit(10) {
            queue.push_back(s.to_digit(10).unwrap());
            if queue.len() == n {
                let prod: u64 = queue.iter().fold(1, |a, &b| a * b as u64);
                if prod > max_prod {
                    max_prod = prod;
                }
                queue.pop_front();
            }
        }
    }
    max_prod
}


pub fn problem_009() -> u64 {
    let n = 1000;

    for i in 0..n {
        for j in i..((n - 1) / 2) {
            let k = n - i - j;
            if (i * i + j * j == k * k) {
                return i * j * k;
            }
        }
    }
    0
}

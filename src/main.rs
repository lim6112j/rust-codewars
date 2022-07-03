fn divisors(m: u64) -> u64 {
    let sum: u64 = (1..(f64::sqrt(m as f64) as u64 + 1))
        .collect::<Vec<u64>>()
        .into_iter()
        .filter(|x| m % x == 0)
        .flat_map(|x| {
            if x == m / x {
                return [x, 0];
            } else {
                return [x, m / x];
            }
        })
        .reduce(|acc, x| acc + x * x)
        .unwrap();
    return sum;
}
fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    fn is_square(squared: (u64, u64)) -> bool {
        let (_, sq) = squared;
        let sqrtval = u64::pow((f64::sqrt(sq as f64)) as u64, 2);
        return sqrtval == sq;
    }
    let res = (m..n + 1)
        .collect::<Vec<u64>>()
        .into_iter()
        .map(|x| -> (u64, u64) { (x, divisors(x)) })
        .filter(|&x| -> bool { is_square(x) })
        .collect();
    return res;
}

fn sum(m: u64) -> u64 {
    return (1..(f64::sqrt(m as f64) as u64 + 1))
        .collect::<Vec<u64>>()
        .into_iter()
        .filter(|x| m % x == 0)
        .flat_map(|x| [x, m / x])
        .inspect(|x| println!("flatmapped {}", x))
        .reduce(|acc, x| acc + x * x)
        .unwrap();
}
fn is_square_test(squared: (u64, u64)) -> bool {
    let (_, sq) = squared;
    let sqrtval = u64::pow((f64::sqrt(sq as f64)) as u64, 2);
    return sqrtval == sq;
}

// best practice
fn sum_squared_divs(n: u64) -> u64 {
    (1..)
        .take_while(|i| i * i <= n)
        .filter(|i| n % i == 0)
        .fold(0, |s, i| {
            s + (if i * i == n {
                n
            } else {
                i * i + (n / i) * (n / i)
            })
        })
}

fn _is_square(n: u64) -> bool {
    let t = (n as f64).sqrt() as u64;
    t * t == n
}

fn _list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    (m..n + 1)
        .map(|i| (i, sum_squared_divs(i)))
        .filter(|&(_, sq)| _is_square(sq))
        .collect()
}

// another best practice
fn __list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    (m..=n)
        .map(|i| {
            (
                i,
                (1..=(i as f32).sqrt() as u64)
                    .filter_map(|d| {
                        let q = i / d;
                        if q * d == i {
                            Some(d * d + if q == d { 0 } else { q * q })
                        } else {
                            None
                        }
                    })
                    .sum(),
            )
        })
        .filter(|t| (t.1 as f64).sqrt().fract() == 0.0)
        .collect::<Vec<_>>()
}
fn main() {
    //let result = is_square_test((1, 1));
    //println!("result => {}", result);
    //let result = sum(676);
    //println!("result => {}", result);
    let result = list_squared(1, 250);
    println!("result => {:?}", result);
}

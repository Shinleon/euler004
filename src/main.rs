
fn main() {
    println!("Hello, world!");
    let (p, (f1, f2)) = palindrome(8);
    println!("palind: {}, factors: {} {}", p, f1, f2);
}

fn ndigit_min_max(n: u8) -> (u64, u64) {
    let smallest = 10_u64.pow((n-1).into());
    let largest = 10_u64.pow(n.into())-1;
    (smallest, largest)
}

fn check_palind(n: u64) -> bool {
    let mut rev = 0;
    let mut temporary = n;
    while temporary > 0 {
        rev = rev*10 + temporary%10;
        temporary = temporary/10;
    }
    rev == n
}

fn palindrome(n: u8) -> (u64, (u64, u64)){
    let (stop, start) = ndigit_min_max(n);
    let mut largest = 0;
    let mut factor1 = 0;
    let mut factor2 = 0;
    for i in (2*stop..=2*start).rev() {
        let mid = (i as f64)/2.0;
        let mut upper = mid.ceil() as u64;
        let mut lower = mid.floor() as u64;
        while (upper * lower > largest) && (upper <= start && lower >= stop) {
            if (upper % 11 == 0 || lower % 11 == 0) && check_palind(upper*lower) {
                largest = upper * lower;
                factor1 = upper;
                factor2 = lower;
            }
            upper += 1;
            lower -= 1;
        }
    }
    (largest, (factor1, factor2))
    
}
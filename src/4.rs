// --- Day 4: Secure Container ---

fn check_double(mut x: i32) -> bool {
    let mut prev = x % 10;
    x = x / 10;
    while x >= 1 {
        let d = x % 10;
        if d == prev {
            return true;
        }
        prev = d;
        x = x / 10;
    }
    false
}

fn check_distinct_double(x: i32) -> bool {
    // when all fails, quick and dirty works
    check_double_substr(&format!("{}", x))
}

fn check_double_substr(s: &str) -> bool {
    // use run-length encoding
    let chars: Vec<char> = s.chars().collect();
    let mut counts = std::collections::HashSet::new();
    let mut count = 1;
    let mut prev = &chars[0];
    for c in &chars[1..] {
        if c != prev {
            counts.insert(count);
            count = 1;
            prev = c;
        } else {
            count += 1;
        }
    }
    counts.insert(count);
    counts.contains(&2)
}

fn never_decrease(mut x: i32) -> bool {
    let mut prev = x % 10;
    x = x / 10;
    while x >= 1 {
        let d = x % 10;
        if d > prev {
            return false;
        }
        prev = d;
        x = x / 10;
    }
    true
}

fn main() {
    println!(
        "Part One: {}",
        (109165..=576723)
            .map(|x| if check_double(x) && never_decrease(x) {
                1
            } else {
                0
            })
            .sum::<i32>()
    );
    println!(
        "Part Two: {}",
        (109165..=576723)
            .map(|x| if check_distinct_double(x) && never_decrease(x) {
                1
            } else {
                0
            })
            .sum::<i32>()
    );
}

#[test]
fn tests() {
    assert_eq!(true, check_double(111111));
    assert_eq!(true, check_double(223450));
    assert_eq!(true, never_decrease(111111));
    assert_eq!(false, never_decrease(223450));
    assert_eq!(true, never_decrease(123789));
    assert_eq!(false, check_double(123789));
    assert_eq!(true, check_distinct_double(112233));
    assert_eq!(false, check_distinct_double(123444));
    assert_eq!(false, check_distinct_double(567777));
    assert_eq!(true, check_distinct_double(111122));
}

#[test]
fn test_check_double_substr() {
    assert_eq!(true, check_double_substr("22"));
}

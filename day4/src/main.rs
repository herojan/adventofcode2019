use std::ops::RangeInclusive;

fn main() -> () {
    let range = 356261..=846303usize;

    part1(range.clone());
    part2(range);
}

fn part1(range: RangeInclusive<usize>) {
    println!(
        "{}",
        range.filter(|&num| check_password(num, |n| n > 1)).count()
    );
}

fn part2(range: RangeInclusive<usize>) {
    println!(
        "{}",
        range.filter(|&num| check_password(num, |n| n == 2)).count()
    );
}

fn check_password<F>(num: usize, f: F) -> bool
where
    F: Fn(usize) -> bool,
{
    if num < 100000 {
        return false;
    }
    let mut in_a_row_count = 1usize;
    let mut iterator = ReverseDigitsIterator { num };
    let mut runs: Vec<usize> = vec![];

    let mut prev = iterator.next().unwrap();

    while let Some(curr) = iterator.next() {
        if prev < curr {
            return false;
        }
        if prev == curr {
            in_a_row_count += 1;
        } else {
            runs.push(in_a_row_count);
            in_a_row_count = 1;
        }

        prev = curr;
    }
    if in_a_row_count > 1 {
        runs.push(in_a_row_count);
    }
    return runs.into_iter().any(f);
}

struct ReverseDigitsIterator {
    num: usize,
}

impl Iterator for ReverseDigitsIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.num == 0 {
            return None;
        }
        let base = 10usize;
        let digit = (self.num % base) as u8;
        self.num /= base;

        Some(digit)
    }
}

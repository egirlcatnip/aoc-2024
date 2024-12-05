use std::iter::zip;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace().flat_map(str::parse::<u32>);
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .unzip();

    left.sort();
    right.sort();

    let sum: u32 = zip(left, right).map(|(a, b)| a.abs_diff(b)).sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace().flat_map(str::parse::<u32>);
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .unzip();

    let similarity_score: u32 = left
        .iter()
        .map(|left| left * right.iter().filter(|right| &left == right).count() as u32)
        .sum();

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

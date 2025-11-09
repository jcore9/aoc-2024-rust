advent_of_code::solution!(2);

#[inline]
fn parse_line(line: &str) -> smallvec::SmallVec<[u8; 8]> {
    line.split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

#[inline]
fn check_values(values: &[u8]) -> bool {
    let mut inc = true;
    let mut dec = true;
    for win in values.windows(2) {
        if win[0].abs_diff(win[1]) > 3 {
            return false;
        }
        inc &= win[0] < win[1];
        dec &= win[0] > win[1];
        if !(inc ^ dec) {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter(|line| check_values(&parse_line(line)))
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter(|line| {
                let values = parse_line(line);
                check_values(&values)
                    || (0..values.len()).any(|i| {
                        let mut values = values.clone();
                        values.remove(i);
                        check_values(&values)
                    })
            })
            .count() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

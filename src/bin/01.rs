advent_of_code::solution!(1);

#[inline]
fn parse_line(line: &str) -> (u32, u32) {
    let (l, r) = line.split_once(|c: char| c.is_ascii_whitespace()).unwrap();
    let l = l.parse().unwrap();
    let r = r.trim_ascii_start().parse().unwrap();
    (l, r)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut left = smallvec::SmallVec::<[u32; 1024]>::new();
    let mut right = smallvec::SmallVec::<[u32; 1024]>::new();
    input.lines().for_each(|line| {
        let (l, r) = parse_line(line);

        left.insert(left.binary_search(&l).unwrap_or_else(|e| e), l);
        right.insert(right.binary_search(&r).unwrap_or_else(|e| e), r);
    });

    let solution: u32 = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    Some(solution as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut left = smallvec::SmallVec::<[u32; 1024]>::new();
    let mut right_count =
        rustc_hash::FxHashMap::with_capacity_and_hasher(896, rustc_hash::FxBuildHasher);
    input.lines().for_each(|line| {
        let (l, r) = parse_line(line);

        left.insert(left.binary_search(&l).unwrap_or_else(|e| e), l);
        *right_count.entry(r).or_insert(0) += 1;
    });

    let solution: u32 = left
        .into_iter()
        .filter_map(|l| right_count.get(&l).map(|count| l * count))
        .sum();

    Some(solution as u64)
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

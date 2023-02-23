pub fn part_one(input: &str) -> Option<u32> {

    input.split("\n\n").
    map(|x| x.split("\n").map(|y| 
        y.parse::<u32>().unwrap()).sum::<u32>())
    .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sorted_sums = input.split("\n\n").
    map(|x| x.split("\n").map(|y| 
        y.parse::<u32>().unwrap()).sum::<u32>())
    .collect::<Vec<u32>>();

    sorted_sums.sort();

    Some(sorted_sums.iter().rev().take(3).sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}

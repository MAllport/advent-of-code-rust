pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim().split('\n')
    .map(|game| (game.chars().nth(0), game.chars().nth(2)))
    .map(|(oppo, hand)| 
        ((oppo.unwrap() as u8 - b'A') as i16, (hand.unwrap() as u8 - b'X') as i16))
    .map(|(oppo, hand)| (3 * (1 + hand - oppo).rem_euclid(3) + hand + 1) as u32)
    .sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(|game|{
        let bytes = game.as_bytes();

        let oppo = (bytes[0] - b'A') as i8;
        let result = (bytes[2] - b'X') as i8;
        let hand_score = (oppo + result - 1).rem_euclid(3) + 1;

        (3 * result + hand_score) as u32
    })
    .sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}

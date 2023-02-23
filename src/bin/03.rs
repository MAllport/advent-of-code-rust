use std::collections::{HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|rucksack|{
        let (left, right) = rucksack.split_at(rucksack.len() / 2);
        let left_set: HashSet<char> = left.chars().collect();
        let right_set: HashSet<char> = right.chars().collect();
        let commons = left_set.intersection(&right_set);

        commons.map(|&c: &char| {
            match c{
                _ if c.is_uppercase() => (c as u8 - b'A' + 27) as u32,
                _ if c.is_lowercase() => (c as u8 - b'a' + 1) as u32,
                _ => panic!("Fail")
            }
        }).sum::<u32>()
    }).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().collect::<Vec<_>>().chunks(3).map(|rucksacks|{
        let mut rucksacks : Vec<HashSet<char>> = rucksacks.iter().map(|r| 
            {r.chars().collect()}).collect();
    
        let (common, others) = rucksacks.split_at_mut(1);
        let common = &mut common[0];
        for other in others {
            common.retain(|e| other.contains(e));
        }

        match common.iter().next().unwrap(){
            c if c.is_uppercase() => (*c as u8 - b'A' + 27) as u32,
            c if c.is_lowercase() => (*c as u8 - b'a' + 1) as u32,
            _ => panic!("Fail")
        }
        }).sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}

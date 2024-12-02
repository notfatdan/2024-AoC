use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let (list1, list2) = parse_input(&mut read_lines("input.txt"));
    let sum: i64 = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("{}", sum);
}

fn part_two() {
    let mut sum: i64 = 0;
    let (list1, list2) = parse_input(&mut read_lines("input.txt"));

    let mut map: HashMap<i64, i64> = HashMap::new();

    for num in list2 {
        map.entry(num).and_modify(|x| *x += 1).or_insert(1);
    }

    for &num in &list1 {
        let count = map.get(&num).unwrap_or(&0);
        sum += num * count;
    }

    println!("{}", sum);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_input(contents: &mut Vec<String>) -> (Vec<i64>, Vec<i64>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in contents.iter_mut() {
        let mut parts = line.split_whitespace();
        if let (Some(left), Some(right)) = (parts.next(), parts.next()) {
            list1.push(left.parse::<i64>().unwrap());
            list2.push(right.parse::<i64>().unwrap());
        }
    }
    list1.sort();
    list2.sort();

    (list1, list2)
}

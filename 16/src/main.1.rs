use std::fs;
use std::ops::RangeInclusive;
use std::string::String;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("could not read the input file")
}

fn parse_field_range(input: &str) -> RangeInclusive<usize> {
    let mut iter = input.splitn(2, '-');
    let start: usize = iter.next().unwrap().parse::<usize>().unwrap();
    let end: usize = iter.next().unwrap().parse::<usize>().unwrap();
    start..=end
}

fn parse_field_ranges(input: &str) -> Vec<RangeInclusive<usize>> {
    input
        .split(" or ")
        .map(|range| parse_field_range(range))
        .collect()
}

fn parse_input_fields_ranges(input: &str) -> Vec<RangeInclusive<usize>> {
    input
        .lines()
        .map(|line| parse_field_ranges(line.splitn(2, ": ").collect::<Vec<&str>>()[1]))
        .flatten()
        .collect()
}

fn parse_ticket(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|number| number.parse::<usize>().unwrap())
        .collect()
}

fn get_tickets(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .skip(1)
        .map(|line| parse_ticket(line))
        .collect()
}

fn get_tickets_error_rate(tickets: &Vec<Vec<usize>>, ranges: &Vec<RangeInclusive<usize>>) -> usize {
    tickets
        .into_iter()
        .flatten()
        .filter(|&number| ranges.iter().filter(|range| range.contains(number)).next() == None)
        .fold(0, |acc, x| acc + x)
}

fn main() {
    let input = read_input("input.txt");
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let input_fields = sections[0];
    // let my_ticket = sections[1];
    let nearby_tickets = sections[2];
    let fields = parse_input_fields_ranges(input_fields);
    let tickets = get_tickets(nearby_tickets);
    println!(
        "error rate is {}",
        get_tickets_error_rate(&tickets, &fields)
    );
}

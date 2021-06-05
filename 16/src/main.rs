use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;
use std::string::String;

type TicketField = Vec<RangeInclusive<usize>>;
type TicketFields<'a> = HashMap<&'a str, TicketField>;
type OrderedTicketFields<'a> = HashMap<&'a str, (TicketField, usize)>;
type Ticket = Vec<usize>;
type Tickets = Vec<Ticket>;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("could not read the input file")
}

fn parse_field_range(input: &str) -> RangeInclusive<usize> {
    let mut iter = input.splitn(2, '-');
    let start: usize = iter.next().unwrap().parse::<usize>().unwrap();
    let end: usize = iter.next().unwrap().parse::<usize>().unwrap();
    start..=end
}

fn parse_field_ranges(input: &str) -> TicketField {
    input
        .split(" or ")
        .map(|range| parse_field_range(range))
        .collect()
}

fn get_ticket_fields(input: &str) -> TicketFields {
    let mut fields: TicketFields = HashMap::new();
    input.lines().for_each(|line| {
        let split = line.split_once(": ").unwrap();
        fields.insert(split.0, parse_field_ranges(split.1));
    });
    fields
}

fn parse_ticket(input: &str) -> Ticket {
    input
        .split(",")
        .map(|number| number.parse::<usize>().unwrap())
        .collect()
}

fn get_tickets(input: &str) -> Tickets {
    input
        .lines()
        .skip(1)
        .map(|line| parse_ticket(line))
        .collect()
}

fn remove_invalid_tickets(tickets: &Tickets, ranges: &TicketFields) -> Tickets {
    let flat_ranges = ranges
        .into_iter()
        .map(|(_, value)| value)
        .flatten()
        .collect::<Vec<&RangeInclusive<usize>>>();

    tickets
        .clone()
        .into_iter()
        .filter(|ticket| {
            ticket
                .into_iter()
                .filter(|field| {
                    flat_ranges
                        .iter()
                        .filter(|range| range.contains(field))
                        .next()
                        != None
                })
                .count()
                == ticket.len()
        })
        .collect::<Tickets>()
}

fn determine_field_order<'a>(
    tickets: &Tickets,
    ranges: &'a TicketFields,
) -> Option<OrderedTicketFields<'a>> {
    let mut ordered_fields: OrderedTicketFields = HashMap::new();

    if tickets.len() == 0 {
        return None;
    }
    // put tickets into columns
    let mut field_sets = (0..ranges.len())
        .map(|col| {
            (
                col,
                tickets
                    .into_iter()
                    .map(move |ticket| ticket[col])
                    .collect::<Vec<usize>>(),
            )
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    while ordered_fields.len() != ranges.len() {
        // iterate through all of the ticket fields
        ranges.into_iter().for_each(|(key, set_ranges)| {
            // try to match a column to field
            let matching_sets = field_sets
                .to_owned()
                .into_iter()
                .filter(|(_, set)| {
                    set.to_owned()
                        .into_iter()
                        // check if every element belongs to any range in the ticket field
                        .filter(|field| {
                            set_ranges
                                .into_iter()
                                .filter(|range| range.contains(field))
                                .count()
                                > 0
                        })
                        .count()
                        == set.len()
                })
                .collect::<Vec<_>>();
            // if only one column matched given range
            if matching_sets.len() == 1 {
                // then put it as ordered
                ordered_fields.insert(key, (set_ranges.to_vec(), matching_sets[0].0));
                // and remove it from iterated vec
                field_sets.remove(
                    field_sets
                        .iter()
                        .position(|(idx, _)| matching_sets[0].0 == *idx)
                        .unwrap(),
                );
            }
        });
        // repeat until there are all fields matched to their position
    }

    return Some(ordered_fields);
}

fn main() {
    let input = read_input("input.txt");
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let input_fields = sections[0];
    let ticket_fields = get_ticket_fields(&input_fields);

    let my_ticket_input = sections[1];
    let my_ticket = &get_tickets(my_ticket_input)[0];

    let nearby_tickets_input = sections[2];
    let nearby_tickets = remove_invalid_tickets(&get_tickets(nearby_tickets_input), &ticket_fields);

    let mut ordered_fields = determine_field_order(&nearby_tickets, &ticket_fields).unwrap();

    let result = ordered_fields
        .into_iter()
        .filter(|(key, _)| key.starts_with("departure"))
        .fold(1, |acc, (_, (_, idx))| acc * my_ticket[idx]);

    println!("result is {}", result);
}

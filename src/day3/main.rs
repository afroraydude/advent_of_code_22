struct RucksackPair {
    left_item: String,
    right_item: String,
    shared_value: char,
    priority: u32,
}

struct TripleRucksack {
    left: String,
    middle: String,
    right: String,
    shared_value: char,
    priority: u32,
}

fn get_priority(a: char) -> u32 {
    // a - z = 1 - 26
    // A - Z = 27 - 52

    match a {
        'a'..='z' => a as u32 - 96,
        'A'..='Z' => a as u32 - 38,
        _ => panic!("Invalid input"),
    }
}

fn main() {
    let mut rucksack_pairs: Vec<RucksackPair> = Vec::new();

    let data = include_str!("data.txt");

    for line in data.lines() {
        // part 1
        let mut split = line.split_at(line.len() / 2);
        let left_item = split.0.to_string();
        let right_item = split.1.to_string();

        println!("{} {}", left_item, right_item);

        let mut shared_value = ' ';
        let mut priority = 0;

        for c in left_item.chars() {
            if right_item.contains(c) {
                shared_value = c;
                println!("Shared value: {}", shared_value);
                break;
            }
        }

        priority = get_priority(shared_value);
        println!("Priority: {}", priority);

        rucksack_pairs.push(RucksackPair {
            left_item,
            right_item,
            shared_value,
            priority,
        });
    }

    // add up the priorities
    let mut total_priority = 0;
    for pair in rucksack_pairs {
        total_priority += pair.priority;
    }

    println!("Total priority: {}", total_priority);

    // part 2
    let mut triple_rucksacks: Vec<TripleRucksack> = Vec::new();

    // for every 3 lines
    for i in (0..data.lines().count()).step_by(3) {
        // first item is the first line
        let first_item = data.lines().nth(i).unwrap().to_string();
        // second item is the second line
        let second_item = data.lines().nth(i + 1).unwrap().to_string();
        // third item is the third line
        let third_item = data.lines().nth(i + 2).unwrap().to_string();

        println!("{} {} {}", first_item, second_item, third_item);

        let mut shared_value = ' ';
        let mut priority = 0;

        for c in first_item.chars() {
            if second_item.contains(c) && third_item.contains(c) {
                shared_value = c;
                println!("Shared value: {}", shared_value);
                break;
            }
        }

        priority = get_priority(shared_value);
        println!("Priority: {}", priority);

        triple_rucksacks.push(TripleRucksack {
            left: first_item,
            middle: second_item,
            right: third_item,
            shared_value,
            priority,
        });
    }

    // add up the priorities
    let mut total_priority = 0;
    for triple in triple_rucksacks {
        total_priority += triple.priority;
    }
    println!("Total priority: {}", total_priority);
}

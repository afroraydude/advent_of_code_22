struct Elf {
    calories: u64
}

fn main() {
    let mut elves: Vec<Elf> = Vec::new();
    // Load the data from the file
    let data = include_str!("./data.txt");

    let mut elf = Elf { calories: 0 };

    // for each line in the data
    for line in data.lines() {
        // see if the line has a number in it
        if let Some(calories) = line.parse::<u64>().ok() {
            // if it does, add to the calories of the elf
            elf.calories += calories;
        } else {
            // if it doesn't, add the elf to the list
            println!("Elf with {} calories", elf.calories);
            elves.push(elf);
            // and start a new elf
            elf = Elf { calories: 0 };
        }
    }

    // now we need to sort the elves by calories
    elves.sort_by(|a, b| a.calories.cmp(&b.calories));

    elves = elves.into_iter().rev().collect();

    // and print the top elf
    println!("Top elf has {} calories", elves.first().unwrap().calories);

    let mut top3: u64 = 0;
    for elf in elves.iter().take(3) {
        top3 += elf.calories;
    }

    println!("Top 3 elves have {} calories", top3);
}

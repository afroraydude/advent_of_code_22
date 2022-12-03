enum rps {
    rock,
    paper,
    scissors,
}

fn get_value(c: char) -> rps {
  match c {
    'A' | 'X' => rps::rock,
    'B' | 'Y' => rps::paper,
    'C' | 'Z' => rps::scissors,
    _ => panic!("Invalid input"),
  }
}

fn get_myvalue(a: &rps, c: char) -> rps {
  match (a, c) {
    (rps::rock, 'X') => rps::scissors,
    (rps::rock, 'Y') => rps::rock,
    (rps::rock, 'Z') => rps::paper,
    (rps::paper, 'X') => rps::rock,
    (rps::paper, 'Y') => rps::paper,
    (rps::paper, 'Z') => rps::scissors,
    (rps::scissors, 'X') => rps::paper,
    (rps::scissors, 'Y') => rps::scissors,
    (rps::scissors, 'Z') => rps::rock,
    _ => panic!("Invalid input"),
  }
}

fn calculate_points(a: &rps, b: &rps) -> u64 {
  match (a, b) {
    (rps::rock, rps::scissors) => 7,
    (rps::paper, rps::rock) => 8,
    (rps::scissors, rps::paper) => 9,
    (rps::rock, rps::paper) => 1,
    (rps::paper, rps::scissors) => 2,
    (rps::scissors, rps::rock) => 3,
    (rps::rock, rps::rock) => 4,
    (rps::paper, rps::paper) => 5,
    (rps::scissors, rps::scissors) => 6,
  }
}

fn main() {
  let mut opp_points: u64 = 0;
  let mut my_points: u64 = 0;

  // load the data.txt file
  let data = include_str!("./data.txt");

  // for each line in the data
  for line in data.lines() {
    // split the line by spaces
    let mut split = line.split_whitespace();
    // get the first character of the first word
    let opp_choice = get_value(split.next().unwrap().chars().next().unwrap());
    // get the first character of the second word
    let my_choice = get_value(split.next().unwrap().chars().next().unwrap());

    // calculate the points for each choice
    let opp_choice_points = calculate_points(&opp_choice, &my_choice);
    let my_choice_points = calculate_points(&my_choice, &opp_choice);

    // add the points to the total
    opp_points += opp_choice_points;
    my_points += my_choice_points;
  }

  // print the results
  println!("Opponent points: {}", opp_points);
  println!("My points: {}", my_points);

  opp_points = 0;
  my_points = 0;

  // restart from first line
  for line in data.lines() {
    // split the line by spaces
    let mut split = line.split_whitespace();
    // get the first character of the first word
    let opp_choice = get_value(split.next().unwrap().chars().next().unwrap());
    // get the first character of the second word
    let my_choice = get_myvalue(&opp_choice, split.next().unwrap().chars().next().unwrap());

    // calculate the points for each choice
    let opp_choice_points = calculate_points(&opp_choice, &my_choice);
    let my_choice_points = calculate_points(&my_choice, &opp_choice);

    // add the points to the total
    opp_points += opp_choice_points;
    my_points += my_choice_points;
  }

  // print the results
  println!("Opponent points: {}", opp_points);
  println!("My points: {}", my_points);
}
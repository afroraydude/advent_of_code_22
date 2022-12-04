struct Assignment {
  min: u32,
  max: u32,
}

fn parse_assignment(data: &str) -> Assignment {
  let mut parts = data.split('-');
  let min = parts.next().unwrap().parse().unwrap();
  let max = parts.next().unwrap().parse().unwrap();
  Assignment { min, max }
}

fn main() {
  // read the data from the file
  let data = include_str!("./data.txt");

  let mut issues_pt1: u32 = 0;
  let mut issues_pt2: u32 = 0;

  for line in data.lines() {
    let pair = line.split(',').collect::<Vec<_>>();
    let assignment_a = parse_assignment(pair[0]);
    let assignment_b = parse_assignment(pair[1]);

    if assignment_a.min <= assignment_b.min && assignment_a.max >= assignment_b.max {
      issues_pt1 += 1;
      println!("{} overlaps {}", pair[0], pair[1]);
    } else if assignment_b.min <= assignment_a.min && assignment_b.max >= assignment_a.max {
      issues_pt1 += 1;
      println!("{} overlaps {}", pair[1], pair[0]);
    }

    // if they overlap at all, they are invalid
    if assignment_a.min <= assignment_b.max && assignment_a.max >= assignment_b.min {
      issues_pt2 += 1;
      println!("{} overlaps {}", pair[0], pair[1]);
    } else if assignment_b.min <= assignment_a.max && assignment_b.max >= assignment_a.min {
      issues_pt2 += 1;
      println!("{} overlaps {}", pair[1], pair[0]);
    }
  }

  println!("Issues: {}", issues_pt1);
  println!("Issues: {}", issues_pt2);
}

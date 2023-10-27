use std::{collections::HashSet, fs};

fn main() {
  let choice = "1";

  let dictionary = fs::read_to_string(format!("./dictionaries/{}.txt", choice))
    .unwrap()
    .to_uppercase();
  let dictionary: HashSet<&str> = dictionary.lines().collect();

  let text = fs::read_to_string("text.txt").unwrap().to_uppercase();
  let text_words: Vec<&str> = text.split_whitespace().collect();
  let words: Vec<&str> = text_words
    .clone()
    .into_iter()
    .collect::<HashSet<&str>>()
    .into_iter()
    .collect();

  let mut map = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
  ];

  fastrand::seed(0);

  let mut last_score = 0;
  let mut tries = 0;
  let mut best_percentage = 0;

  loop {
    let first_place = fastrand::usize(..26);
    let second_place = fastrand::usize(..26);

    let mut new_map = map;

    new_map[first_place] = map[second_place];
    new_map[second_place] = map[first_place];

    let mut score = 0;

    for &word in &words {
      let new_word: String = word.chars().map(|c| new_map[c as usize - 65]).collect();
      if dictionary.contains(new_word.as_str()) {
        score += 1;
      }
    }

    let percentage = 100 * score / words.len();

    if score >= last_score {
      map = new_map;
      last_score = score;
    }

    if percentage > best_percentage {
      let map_string = new_map.iter().collect::<String>();
      println!("{}% | Try {} | {}", percentage, tries, map_string);
      best_percentage = percentage;
    }

    let cutoff = 2000;
    if tries > cutoff {
      if best_percentage >= 95 {
        break;
      }

      tries = 0;
      last_score = 0;
      fastrand::shuffle(&mut map);
    }

    tries += 1;
  }

  let mut new_words = vec![];

  for &word in &text_words {
    let new_word: String = word.chars().map(|c| map[c as usize - 65]).collect();
    new_words.push(new_word);
    // let has = dictionary.contains(new_word.as_str());
    // if !has {
    //   println!("{}", new_word);
    // }
  }

  let new_text = new_words.join(" ");

  println!("{}", new_text);
}

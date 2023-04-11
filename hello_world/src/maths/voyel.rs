

pub fn get_voyel(s: String) -> i16 {
    let mut result = 0;
    let voyels = "aeiouy";

    for c in s.chars() {
        if (voyels.contains(c)) {
            result += 1;
        }
    }
    return result;
}


pub fn voyel(s: &str) -> usize {
    let voyels = ['a', 'e', 'i', 'o', 'u', 'y'];
    s.chars().filter(|c| voyels.contains(c)).count()
}

pub fn scrabble_score(s: &str) -> i32 {
    s.chars()
        .map(|c| match c.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
            'D' | 'G' => 2,
            'B' | 'C' | 'M' | 'P' => 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            'K' => 5,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
            _ => 0,
        })
        .sum()
}

// pub fn scrabble_score_(s: &str) -> i32 {
//     let scores = [
//         (['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'], 1),
//         (['D', 'G'], 2),
//         (['B', 'C', 'M', 'P'], 3),
//         (['F', 'H', 'V', 'W', 'Y'], 4),
//         (['K'], 5),
//         (['J', 'X'], 8),
//         (['Q', 'Z'], 10),
//     ];
//     s.chars()
//         .map(|c| {
//             scores
//                 .iter()
//                 .find(|(letters, _)| letters.contains(&c.to_ascii_uppercase()))
//                 .map(|(_, score)| score)
//                 .unwrap_or(&0)
//         })
//         .sum()
// }













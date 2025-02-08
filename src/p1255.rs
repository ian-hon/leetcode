pub fn run() {
    for i in [
        (
            vec!["dog".to_string(), "cat".to_string(), "dad".to_string(), "good".to_string()],
            vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
            vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        )
    ] {
        println!("{}", max_score_words(i.0, i.1, i.2));
    }
}

pub fn max_score_words(words   : Vec<String>, 
                        letters : Vec<char>, 
                        score   : Vec<i32>) 
    -> i32 
{
    fn idx(c: char) -> usize { 
        (c as u8 - b'a') as usize 
    }
    let mut counts = [0i8; 26];

    letters.into_iter().for_each(|c| counts[idx(c)] += 1);

    let mut sets = vec![(0, counts)];
    let mut best = 0;

    for word in words {
        'setloop: for set_idx in 0..sets.len() {
            let (mut val, mut set) = sets[set_idx];

            for ch_idx in word.chars().map(|ch| idx(ch)) {
                if set[ch_idx] > 0 {
                    set[ch_idx] -= 1;
                    val += score[ch_idx];
                } else {
                    continue 'setloop;
                }
            }
            best = best.max(val);
            sets.push((val, set));
        }
    }
    best
}
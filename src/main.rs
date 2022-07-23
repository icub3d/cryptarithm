use std::collections::HashSet;
use std::env;
use std::str;

use dinglebit_combinatorics::{Combination, Permutation};

fn main() {
    let numbers = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    // TODO include operators, right now we just add and assume 3 args.
    let mut letters = HashSet::new();
    let mut words = vec![];
    let mut op = "+".to_string();
    for (i, arg) in env::args().skip(1).enumerate() {
        let arg = arg.to_lowercase();
        if i == 1 {
            op = arg.clone();
            continue;
        } else if i == 3 {
            continue;
        }
        words.push(arg.clone());
        for c in arg.chars() {
            letters.insert(c);
        }
    }
    let letters: Vec<char> = letters.iter().cloned().collect();

    for cc in Combination::new(numbers.len(), letters.len()) {
        for pp in Permutation::new(cc.len()) {
            // replace all letters with this combinations numbers.
            let mut nn = vec![];
            for (i, n) in words.iter().enumerate() {
                nn.push(n.clone());
                for (j, p) in pp.iter().enumerate() {
                    nn[i] = str::replace(&nn[i], letters[j], numbers[cc[*p]]);
                }
            }
            if check(&nn[0], &op, &nn[1], &nn[2]) {
                println!("{} + {} = {}", nn[0], nn[1], nn[2]);
                break;
            }
        }
    }
}

fn check(a: &str, op: &str, b: &str, c: &str) -> bool {
    let a = a.trim_start_matches('0');
    let b = b.trim_start_matches('0');
    let c = c.trim_start_matches('0');
    if op == "+" {
        a.parse::<i32>().unwrap() + b.parse::<i32>().unwrap() == c.parse::<i32>().unwrap()
    } else {
        a.parse::<i32>().unwrap() - b.parse::<i32>().unwrap() == c.parse::<i32>().unwrap()
    }
}

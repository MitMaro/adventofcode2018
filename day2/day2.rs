use std::io::{self, BufRead};

fn main() {
	println!("Day 2: Inventory Management System");
	let stdin = io::stdin();
	let mut count_twos = 0;
	let mut count_threes = 0;
	let mut characters: [i32; 26];
	let mut entries: Vec<String> = vec![];

	for input in stdin.lock().lines() {
		entries.push(input.unwrap())
	}

	for line in entries.iter() {
		characters = [0; 26];
		for c in line.chars() {
			let char_index: usize = (c as usize) - ('a' as usize);
			characters[char_index] += 1;
		}

		let mut twos = false;
		let mut threes = false;
		for c_count in characters.iter() {
			if *c_count == 2 {
				twos = true;
			}
			else if *c_count == 3 {
				threes = true;
			}
			if twos && threes {
				break;
			}
		}
		if twos {
			count_twos += 1;
		}
		if threes {
			count_threes += 1;
		}
	}

	println!("Part  I: {}", count_twos * count_threes);

	let mut id:String = String::from("");
	'source: for (i, source) in entries.iter().enumerate() {
		'target: for target in entries[(i+1)..].iter() {
			let mut different = 0;
			for (j, (c, t)) in source.chars().zip(target.chars()).enumerate() {
				if c != t {
					if different != 0 {
						continue 'target;
					}
					different = j + 1;
				}
			}
			let mut source = source.to_string();
			source.remove(different - 1);
			if different != 0 {
				id = source;
				break 'source;
			}
		}
	}

	println!("Part II: {}", id)
}
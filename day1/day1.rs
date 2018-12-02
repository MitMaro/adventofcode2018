use std::io::{self, BufRead};
use std::collections::HashSet;

struct Entry {
	is_add: bool,
	value: i32,
}

fn main() {
	println!("Day 1: Chronal Calibration");
	let stdin = io::stdin();
	let mut frequency = 0;
	let mut frequencies: HashSet<i32> = HashSet::new();
	let mut entries: Vec<Entry> = vec![];

	for input in stdin.lock().lines() {
		let line = input.unwrap();
		let value = line.get(1..).unwrap().parse::<i32>().unwrap();
		entries.push(Entry {
			is_add: line.starts_with('+'),
			value,
		})
	}

	// part 1
	for entry in entries.iter() {
		if entry.is_add {
			frequency += entry.value;
		} else {
			frequency -= entry.value;
		}
	}
	println!("Part  I: {}", frequency);

	frequency = 0;
	// part 2
	'reloop: loop {
		for entry in entries.iter() {
			if entry.is_add {
				frequency += entry.value;
			} else {
				frequency -= entry.value;
			}
			if frequencies.contains(&frequency) {
				break 'reloop;
			}
			frequencies.insert(frequency);
		}
	}
	println!("Part II: {}", frequency);
}
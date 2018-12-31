use std::io::{self, BufRead};

struct Entry {
	id: i32,
	position_x: i32,
	position_y: i32,
	width: i32,
	height: i32,
}

fn main() {
	println!("Day 3: No Matter How You Slice It");
	let stdin = io::stdin();
	let mut entries: Vec<Entry> = vec![];
	let mut fabric: [i32; 1000*1000] = [0; 1000*1000];

	for input in stdin.lock().lines() {
		let line = input.unwrap();
		let values: Vec<&str> = line.split_whitespace().into_iter().collect();
		let claim_id = values[0].replace('#', "").parse().unwrap();
		let position: Vec<i32> = values[2]
			.split(',')
			.map(|v| v.replace(':', "").parse().unwrap())
			.collect();
		let size: Vec<i32> = values[3].split('x').map(|v| v.parse().unwrap()).collect();
		let (position_x, position_y) = (position[0], position[1]);
		let (size_width, size_height) = (size[0], size[1]);

		entries.push(Entry {
			id: claim_id,
			position_x,
			position_y,
			width: size_width,
			height: size_height,
		});
	}

	for entry in entries.iter() {
		for x in 0..entry.width {
			for y  in 0..entry.height {
				let index = ((x+entry.position_x)*1000 + (y+entry.position_y)) as usize;
				fabric[index] += 1;
			}
		}
	}

	let mut ok_claim = 0;
	for entry in entries.iter() {
		let mut is_overlap = false;
		for x in 0..entry.width {
			for y  in 0..entry.height {
				let index = ((x+entry.position_x)*1000 + (y+entry.position_y)) as usize;
				if fabric[index] > 1 {
					is_overlap = true;
				}
			}
		}

		if !is_overlap {
			println!("{}", entry.id);
			ok_claim = entry.id;
		}
	}

	let mut overlap = 0;
	for x in 0..1000 {
		for y  in 0..1000 {
			if fabric[(x*1000 + y) as usize] > 1 {
				overlap += 1;
			}
		}
	}

	println!("Part  I: {}", overlap);

	println!("Part II: {}", ok_claim)
}
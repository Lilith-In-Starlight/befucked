#[allow(unreachable_code)]
use std::collections::HashMap;
use std::io;
use std::fs;

#[derive(Clone)]
struct ProgramPos {
	x: usize,
	y: usize,
	old_pos: (usize, usize),
}

fn main() {
    let program = fs::read_to_string("main.bfg").unwrap();
	let program = program.replace("\r", "");
	let program_lines = program.split("\n").collect::<Vec<_>>();
	let mut pos = ProgramPos { x: 0, y: 0, old_pos: (0, 0) };
	let mut set_initial = false;
	let mut y = 0;
	for line in &program_lines {
		for (x, ch) in line.char_indices() {
			if ch == 'N' {
				if !set_initial {
					set_initial = true;
					pos.x = x;
					pos.y = y;
				} else {
					panic!("Error: Found too many starting points.");
				}
			}
		}
		y += 1;
	}

	let mut data_space: HashMap<i64,i64> = HashMap::new();
	run_branch(&mut data_space, &mut pos, &program_lines, (0, false, 0), &Vec::new());
	println!("Program exits without errors.");
}

fn run_branch(data_space: &mut HashMap<i64, i64>, pos: &mut ProgramPos, program_lines: &Vec<&str>, prev_pointer: (i64, bool, i64), hard_exclude: &Vec<(usize, usize)>) -> (Option<i64>, i64, bool) {
	let mut d_pos: i64 = prev_pointer.0;
	let mut in_values: bool = prev_pointer.1;
	let mut contained: i64 = prev_pointer.2;
	let mut out: Option<i64> = None;
	let mut fstep: bool = true;

	'beloop:loop {
		let current = char_at(pos.x, pos.y, &program_lines);
		let neigh_excl = {
			if !fstep {
				Vec::from([pos.old_pos])
			} else {
				let mut k = Vec::from([pos.old_pos]);
				for i in hard_exclude.iter() {
					k.push(i.to_owned());
				}
				k
			}
		};
		fstep = false;
		let neigh = get_neighbors(pos.x, pos.y, &program_lines, neigh_excl);
		let mut new_pos = (pos.x, pos.y);
		match current {
			'N' => {
				if neigh.len() > 1 { panic!("Start element doesn't know where to go") }
				else if neigh.len() == 1 { new_pos = (neigh[0].0, neigh[0].1) }
				else { break 'beloop }
			},
			'>' => {
				d_pos += 1;
				if neigh.len() > 1 { panic!("Error at ({}, {}): MRight doesn't know where to go!", pos.x+1, pos.y+1) }
				else if neigh.len() == 1 { new_pos = (neigh[0].0, neigh[0].1) }
				else { break 'beloop }
			},
			'<' => {
				d_pos -= 1;
				if neigh.len() > 1 { panic!("Error at ({}, {}): MLeft doesn't know where to go!", pos.x+1, pos.y+1) }
				else if neigh.len() == 1 { new_pos = (neigh[0].0, neigh[0].1) }
				else { break 'beloop }
			},
			'G' => {
				if in_values { contained = d_pos }
				else { 
					let k = get_data(&data_space, &d_pos);
					data_space.insert(d_pos, contained);
					contained = k;
				}
				if neigh.len() > 1 { panic!("Error at ({}, {}): Grab doesn't know where to go!", pos.x+1, pos.y+1) }
				else if neigh.len() == 1 { new_pos = (neigh[0].0, neigh[0].1) }
				else { break 'beloop }
			},
			'^' => {
				in_values = !in_values;
				if neigh.len() > 1 { panic!("Error at ({}, {}): MUp doesn't know where to go!", pos.x+1, pos.y+1) }
				else if neigh.len() == 1 { new_pos = (neigh[0].0, neigh[0].1) }
				else { break 'beloop }
			},
			'E' => {
				println!("{}", &contained);
				if neigh.len() > 1 { panic!("Error at ({}, {}): Print doesn't know where to go!", pos.x+1, pos.y+1) }
				else if neigh.len() == 1 { new_pos = (neigh[0].0, neigh[0].1) }
				else { break 'beloop }
			},
			'O' => {
				if neigh.len() > 1 { println!("Warning at ({}, {}): Unreachable code!", pos.x+1, pos.y+1) }
				if in_values { out = Some(d_pos) }
				else { out = Some(get_data(data_space, &d_pos)) }
				break 'beloop
			},
			'R' => {
				if pos.old_pos.0 == pos.x && pos.old_pos.1 != pos.y {
					panic!("Error at ({}, {}): Repeat can only run from a side", pos.x+1, pos.y+1);
				}
				if char_at(pos.x, pos.y - 1, program_lines) == ' ' || char_at(pos.x, pos.y + 1, program_lines) == ' ' {
					panic!("Error at ({}, {}): Repeat requires a branch above and a branch below", pos.x+1, pos.y+1);
				}
				let mut top_pos = ProgramPos { x: pos.x, y: pos.y - 1, old_pos: (pos.x, pos.y) };
				let iterexcl: Vec<(usize, usize)> = Vec::from([(pos.x-1, pos.y), (pos.x+1, pos.y)]);
				let iteramt = run_branch(data_space, &mut top_pos, program_lines, (d_pos, in_values, contained), &iterexcl);
				match iteramt.0 {
					None => panic!("Error at ({}, {}): Repeat's top branch must return a value", pos.x+1, pos.y+1),
					Some(x) => {
						if x < 0 {
							panic!("Error at ({}, {}): Can't repeat a negative amount of times", pos.x+1, pos.y+1);
						}
						for _ in 0..x {
							let mut bot_pos = ProgramPos { x: pos.x, y: pos.y + 1, old_pos: (pos.x, pos.y) };
							let iterproc = run_branch(data_space, &mut bot_pos, program_lines, (d_pos, in_values, contained), &iterexcl);
							d_pos = iterproc.1;
							in_values = iterproc.2;
						}
					}
				}
				if in_values { contained = d_pos }
				else { contained = get_data(&data_space, &d_pos) }
				if pos.old_pos.0 > pos.x {
					new_pos = (pos.x - 1, pos.y);
				} else {
					new_pos = (pos.x + 1, pos.y);
				}
				if char_at(new_pos.0, new_pos.1, program_lines) == ' ' { break 'beloop }
			},
			'-' => {
				if pos.old_pos.0 < pos.x { new_pos = (pos.x + 1, pos.y)
				} else if pos.old_pos.0 > pos.x { new_pos = (pos.x - 1, pos.y) }
				else if neigh.len() == 1 { new_pos = (neigh[0].0, neigh[0].1) }
				else { panic!("Error at ({}, {}): Control flow doesn't know where to go!", pos.x+1, pos.y+1) }
			},
			'|' => {
				if pos.old_pos.1 < pos.y { new_pos = (pos.x, pos.y + 1) }
				else if pos.old_pos.1 > pos.y { new_pos = (pos.x, pos.y - 1) }
				else if neigh.len() == 1 { new_pos = (neigh[0].0, neigh[0].1) }
				else { panic!("Error at ({}, {}): Control flow doesn't know where to go!", pos.x+1, pos.y+1) }
			},
			'#' => {
				if neigh.len() > 1 { panic!("Error at ({}, {}): UIn doesn't know where to go!", pos.x+1, pos.y+1) }
				else if neigh.len() == 1 { new_pos = (neigh[0].0, neigh[0].1) }
				let mut input = String::new();
				println!("Program awaiting input...");
				io::stdin().read_line(&mut input).unwrap();
				let n = input.trim().parse::<i64>();
				match n {
					Ok(x) => {
						if in_values {
							println!("Warning at ({}, {}): Attempted to write at immutable position", pos.x+1, pos.y+1);
						}
						else {
							data_space.insert(d_pos, x);
						}
					}
					_ => {
						if input.trim().as_bytes().len() != 1 {
							panic!("OVERFLOW: Input must be either a number or a single byte");
						} else {
							data_space.insert(d_pos, input.chars().nth(0).unwrap() as i64);
						}
					},
				}
			},
			'X' => {
				let mut branchpos = ProgramPos { x: pos.x - 1, y: pos.y - 1, old_pos: (pos.x, pos.y) };
				let c1branch = run_branch(data_space, &mut branchpos, program_lines, (d_pos, in_values, contained), &Vec::new());
				let mut branchpos = ProgramPos { x: pos.x - 1, y: pos.y + 1, old_pos: (pos.x, pos.y) };
				let c2branch = run_branch(data_space, &mut branchpos, program_lines, (d_pos, in_values, contained), &Vec::new());
				if let Some(x) = c1branch.0 {
					if let Some(y) = c2branch.0 {
						if x == y {
							new_pos = (pos.x + 1, pos.y - 1);
						} else {
							new_pos = (pos.x + 1, pos.y + 1);
						}
						if char_at(new_pos.0, new_pos.1, program_lines) == ' ' { break 'beloop }
					} else {
						panic!("Error at ({}, {}): Cond's left right branch doesn't have an output!", pos.x+1, pos.y+1);
					}
				} else {
					panic!("Error at ({}, {}): Cond's top left branch doesn't have an output!", pos.x+1, pos.y+1);
				}
			},
			'A' => {
				if neigh.len() > 1 { panic!("Error at ({}, {}): Absol doesn't know where to go!", pos.x+1, pos.y+1) }
				else if neigh.len() == 1 {
					new_pos = (neigh[0].0, neigh[0].1);
					if in_values {
						d_pos = d_pos.abs();
					} else {
						d_pos = get_data(data_space, &d_pos).abs();
					}
				}
				else { break 'beloop }
			},
			'T' => {
				if char_at(pos.x, pos.y+1, program_lines) != ' ' { new_pos = (pos.x, pos.y + 1) }
				else { break 'beloop }
			},
			' ' => panic!("Error at ({}, {}): Program was led to whitespace\nLast position was ({}, {})", pos.x+1, pos.y+1, pos.old_pos.0+1, pos.old_pos.1+1),
			_ => panic!("Error at ({}, {}): Unknown element {}\nLast position was ({}, {})", pos.x+1, pos.y+1, char_at(pos.x, pos.y, program_lines), pos.old_pos.0, pos.old_pos.1),
		}
		pos.old_pos = (pos.x, pos.y);
		pos.x = new_pos.0;
		pos.y = new_pos.1;
	}
	(out, d_pos, in_values)
}

fn char_at(x: usize, y: usize, program_lines: &Vec<&str>) -> char {
	if y >= program_lines.len() { return ' '; }
	let line = program_lines[y];
	if x > line.len() { return ' ' }
	match line.chars().nth(x) {
		Some(x) => return x,
		None => return ' '
	}
}


fn get_neighbors(x: usize, y: usize, program_lines: &Vec<&str>, excl: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
	let mut ret: Vec<(usize, usize)> = Vec::new();
	let low_bound_x = {
		if x > 0 { x-1 }
		else { x }
	};
	let low_bound_y = {
		if y > 0 { y-1 }
		else { y }
	};
	for i in low_bound_x..x+2 {
		for j in low_bound_y..y+2 {
			if ((i as i64)-(x as i64)).abs() == ((j as i64)-(y as i64)).abs() {continue }
			if !(i == x && j == y) && !excl.contains(&&(i, j)) {
				if char_at(i, j, program_lines) != ' ' {
					ret.push((i, j));
				}
			}
		}
	}
	ret
}

fn get_data(data_space: &HashMap<i64, i64>, d_pos: &i64) -> i64 {
	if data_space.contains_key(d_pos) {
		return data_space[d_pos];
	}
	0
}

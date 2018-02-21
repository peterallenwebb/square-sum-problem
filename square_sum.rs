
use std::time::SystemTime;
use std::collections::HashSet;
use std::cmp;


fn main() {
	let search_start = 1;
	let search_limit = 1000;

	let start = SystemTime::now();
	for n in search_start..search_limit {

		let mut reach: Vec<i32> = vec![n; n as usize];

		let dur = SystemTime::now().duration_since(start).expect("time went backwards");

		println!("n = {} @ {} seconds", n, dur.as_secs());

		let mut sln = vec![0; n as usize];

		let peak = find_sqr_seq(&mut reach, 0, 0, &mut sln);

		if peak != n && n > 24 {
			panic!("impossible!");
		}

		println!();
	}
}

fn find_sqr_seq(reach: &mut Vec<i32>, level: i32, last: i32, sln: &mut Vec<i32>) -> i32 {
	let n = reach.len() as i32;

	if level == n {
		check_and_write(sln);
		return 0;
	}

	let mut peak = 0;
	let mut guesses = Vec::new();

	if level == 0 {
		guesses = (0..n).collect();
	} else {
		let m = (2.0 * n as f64 - 1.0).sqrt() as i32 + 1;
		for s in 2..m {
			let i = s * s - last - 2;

			if i >= 0 && 
			   i < n && 
			   level + reach[i as usize] >= n {
			   	guesses.push(i);
			}
		}
	}

	if guesses.len() == 0 {
        return 0;
	}

	if n - level > 5 {
		 guesses.sort_by_key(|i| num_pos(reach, level + 1, *i));
	}

	for i in guesses {
		let mut next_reach = get_next_reach(reach, i as i32);

		sln[level as usize] = (i + 1) as i32;

		peak = cmp::max(peak, 1 + find_sqr_seq(&mut next_reach, level + 1, i as i32, sln));

        // This check returns if we've found a solution. Remove to
        // enumerate all solutions instead.
		if peak + level == n {
			return peak;
		}
	}

	return peak;
}

fn num_pos(reach: &Vec<i32>, level: i32, last: i32) -> i32 {
	let mut num_pos = 0;
	let n = reach.len() as i32;

	if level == 0 {
		for i in 0..n {
			if i != last && (level + reach[i as usize] - 1 >= n) {
				num_pos += 1;
			}
		}
	} else {
		let m = (2.0 * n as f64 + 1.0).sqrt() as i32 + 1;
		for s in 2..m {
			let i = s * s - last - 2;

			if i >= n {
				break;
			}

			if i >= 0 && i != last && (level + reach[i as usize] - 1 >= n) {
				num_pos += 1;
			}
		}
	}

	return num_pos;
}

fn get_next_reach(reach: &Vec<i32>, pick: i32) -> Vec<i32> {
	let mut next_reach: Vec<i32> = Vec::new();

	for j in 0..reach.len() {
		next_reach.push(reach[j] - 1);
	}

	next_reach[pick as usize] = 0;

	return next_reach;
}

fn check_and_write(sln: & Vec<i32>) {
	if !check_sln(sln) {
		panic!("DUD");
	}

	println!("{:?}", sln);
}

fn check_sln(sln: & Vec<i32>) -> bool {
	let mut used = HashSet::new();

	for i in 0..sln.len() {
		used.insert(sln[i]);

		if i > 0 && !is_perfect_sqr(sln[i] + sln[i - 1]) {
			return false;
		}
	}

	return used.len() == sln.len();
}

fn is_perfect_sqr(n: i32) -> bool {
	let mut res = false;
	let h = n & 0xF;

	if h > 9 {
		res = false;
	} else if h != 2 && h != 3 && h != 5 && h != 6 && h != 7 && h != 8 {
		let t = ((n as f64).sqrt() + 0.5).floor() as i32;
		res = t * t == n;
	}

	return res;
}

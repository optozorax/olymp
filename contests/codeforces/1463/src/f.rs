// Just struct to carry info, because Rust make it harder to use global variables
#[derive(Debug)]
struct Info {
	x: u8,
	y: u8,
	size: u8,
	periodic_count: u32,
	remains_count: u8,
}

#[allow(clippy::too_many_arguments)]
fn solve(
	pos: u8,
	mut can_use: BitVec64,
	current_periodic: u8,
	current_remains: u8,
	mut can_take_count: u8,
	max_periodic: &mut u8,
	max_remains: &mut u8,
	i: &Info,
) {
	// Take element at this position
	let current_periodic = current_periodic + 1;
	let current_remains = current_remains + (pos < i.remains_count) as u8;

	let xp = (pos + i.x) % i.size;
	let yp = (pos + i.y) % i.size;

	if xp > pos && can_use.get_bit(xp) {
		can_take_count -= 1;
	}
	if yp > pos && can_use.get_bit(yp) {
		can_take_count -= 1;
	}

	can_use.set_false(xp);
	can_use.set_false(yp);

	// For each next element
	for pos in pos + 1..i.size {
		if can_use.get_bit(pos) {
			// Take element at this position
			solve(pos, can_use, current_periodic, current_remains, can_take_count, max_periodic, max_remains, i);
		}
		can_take_count -= 1;

		// KEY THING: early return by maximum
		// If we cannot in any way be better, than maximum, then return
		if *max_periodic > current_periodic + can_take_count {
			return;
		}
	}

	// When done, compare result
	if *max_periodic as u32 * i.periodic_count + (*max_remains as u32)
		< current_periodic as u32 * i.periodic_count + current_remains as u32
	{
		*max_periodic = current_periodic;
		*max_remains = current_remains;
	}
}

#[fastio::fastio]
pub fn main() {
	let n = read!(u32);
	let x = read!(u8);
	let y = read!(u8);

	let can_use = BitVec64(0xFFFF_FFFF_FFFF_FFFF);
	let i = Info { x, y, size: x + y, periodic_count: n / (x + y) as u32, remains_count: (n % (x + y) as u32) as u8 };
	let mut max_remains = 0;
	let mut max_periodic = 0;
	solve(0, can_use, 0, 0, i.size, &mut max_periodic, &mut max_remains, &i);

	println!("{}", max_periodic as u32 * (n / (x + y) as u32) + max_remains as u32);
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/bit64.rs");

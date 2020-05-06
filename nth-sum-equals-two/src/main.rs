fn main() {
	let now = std::time::Instant::now();
	println!("Answer: {}", olymp::main_task::calc(100));
	println!("Elapsed time: {:?}", now.elapsed());
}
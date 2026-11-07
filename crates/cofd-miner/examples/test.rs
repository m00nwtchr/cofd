#[cfg(not(feature = "embed_meta"))]
fn main() {}

#[cfg(feature = "embed_meta")]
mod real {
	use std::path::PathBuf;

	use cofd_miner::parse_book;

	pub fn run() {
		let args: Vec<_> = std::env::args().collect();
		let path = PathBuf::from(args.get(1).unwrap());

		let b = parse_book(path).unwrap();
		print!("{}", serde_json::ser::to_string_pretty(&b).unwrap());
	}
}

#[cfg(feature = "embed_meta")]
fn main() {
	real::run();
}

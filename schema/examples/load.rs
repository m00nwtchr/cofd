use cofd_schema::book::Book;
use std::fs::File;

fn main() {
	let path = std::env::args().nth(1).expect("Usage: load <path>");

	let book: Book =
		ron::de::from_reader(File::open(path).expect("")).expect("metadata parse error");

	println!("{:#?}", book);
}

use std::process::Command;

fn pnpm_installed() -> bool {
	Command::new("pnpm")
		.arg("-v")
		.status()
		.is_ok_and(|r| r.success())
}

fn tailwindcss_installed() -> bool {
	Command::new("tailwindcss")
		.arg("-h")
		.status()
		.is_ok_and(|r| r.success())
}

const TAILWIND_ARGS: [&'static str; 7] = [
	"-i",
	"input.css",
	"-c",
	"tailwind.config.js",
	"-o",
	"assets/tailwind.css",
	"--minify",
];

fn main() {
	println!("cargo:rerun-if-changed=./src");
	println!("cargo:rerun-if-changed=input.css");

	let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR missing");

	// Compile TailwindCSS .css file
	if pnpm_installed() {
		Command::new("pnpm")
			.args(["dlx", "@tailwindcss/cli"])
			.args(TAILWIND_ARGS)
			.current_dir(&manifest_dir)
			.env("NODE_ENV", "production")
			.spawn()
			.unwrap();
	} else if tailwindcss_installed() {
		Command::new("tailwindcss")
			.args(TAILWIND_ARGS)
			.current_dir(&manifest_dir)
			.env("NODE_ENV", "production")
			.spawn()
			.unwrap();
	} else {
		panic!("Neither `pnpm` nor `tailwindcss` is available; cannot build Tailwind CSS.");
	}
}

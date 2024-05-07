pub mod wound;
mod character;
mod splat;

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero(n: &u16) -> bool {
	*n == 0
}
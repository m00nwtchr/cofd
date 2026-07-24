#[allow(clippy::ptr_arg)]
pub fn is_empty(str: &String) -> bool {
	str.is_empty()
}

#[allow(clippy::ptr_arg)]
pub fn is_empty_vec<T>(vec: &Vec<T>) -> bool {
	vec.is_empty()
}

// #[allow(clippy::trivially_copy_pass_by_ref)]
// pub fn is_five(n: &u16) -> bool {
// 	*n == 5
// }
//
// #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
// pub fn add(a: u16, b: i16) -> u16 {
// 	let res = i32::from(a) + i32::from(b);
//
// 	if res >= 255 {
// 		u16::MAX
// 	} else if res <= 0 {
// 		u16::MIN
// 	} else {
// 		res as u16
// 	}
// }

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero(n: &u8) -> bool {
	*n == 0
}

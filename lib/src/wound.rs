use std::ops::Index;
use serde::{Deserialize, Serialize};

use crate::is_zero;

#[derive(Default, Debug, Copy, Clone)]
pub enum Wound {
	#[default]
	None,
	Bashing,
	Lethal,
	Aggravated,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct WoundTracker {
	#[serde(skip_serializing_if = "is_zero")]
	aggravated: u16,
	#[serde(skip_serializing_if = "is_zero")]
	lethal: u16,
	#[serde(skip_serializing_if = "is_zero")]
	bashing: u16,
}

impl WoundTracker {
	pub fn get(&self, wound: &Wound) -> u16 {
		match wound {
			Wound::None => 0,
			Wound::Bashing => self.bashing,
			Wound::Lethal => self.lethal,
			Wound::Aggravated => self.aggravated,
		}
	}

	pub fn total(&self) -> u16 {
		self.bashing + self.lethal + self.aggravated
	}

	pub fn decrement(&mut self, wound: &Wound) {
		match wound {
			Wound::None => {}
			Wound::Bashing => {
				if self.bashing > 0 {
					self.bashing -= 1;
				}
			}
			Wound::Lethal => {
				if self.lethal > 0 {
					self.lethal -= 1;
				}
			}
			Wound::Aggravated => {
				if self.aggravated > 0 {
					self.aggravated -= 1;
				}
			}
		}
	}

	pub fn increment(&mut self, wound: &Wound) {
		match wound {
			Wound::None => {}
			Wound::Bashing => self.bashing += 1,
			Wound::Lethal => self.lethal += 1,
			Wound::Aggravated => self.aggravated += 1,
		}
	}

	pub fn poke(&mut self, wound: &Wound) {
		match wound {
			Wound::None => self.bashing += 1,
			Wound::Bashing => {
				if self.bashing > 0 {
					self.bashing -= 1;
				}
				self.lethal += 1;
			}
			Wound::Lethal => {
				if self.lethal > 0 {
					self.lethal -= 1;
				}
				self.aggravated += 1;
			}
			Wound::Aggravated => {
				if self.aggravated > 0 {
					self.aggravated -= 1;
				}
			}
		}
	}
}

impl Index<usize> for WoundTracker {
	type Output = Wound;

	fn index(&self, index: usize) -> &Self::Output {
		if index < self.aggravated as usize {
			&Wound::Aggravated
		} else if index >= self.aggravated as usize && index < (self.aggravated + self.lethal) as usize {
			&Wound::Lethal
		} else if index >= (self.aggravated + self.lethal) as usize
			&& index < (self.aggravated + self.lethal + self.bashing) as usize
		{
			&Wound::Bashing
		} else {
			&Wound::None
		}
	}
}
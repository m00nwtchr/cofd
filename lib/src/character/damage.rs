use std::ops::Index;

use serde::{Deserialize, Serialize};

use crate::util::is_zero;

#[derive(Default, Debug, Clone, Copy)]
pub enum Wound {
	#[default]
	None,
	Bashing,
	Lethal,
	Aggravated,
}

impl Wound {
	#[must_use]
	pub fn upgrade(self) -> Wound {
		match self {
			Wound::None => Wound::Bashing,
			Wound::Bashing => Wound::Lethal,
			Wound::Lethal | Wound::Aggravated => Wound::Aggravated,
		}
	}

	#[must_use]
	pub fn downgrade(self) -> Wound {
		match self {
			Wound::Bashing | Wound::None => Wound::None,
			Wound::Lethal => Wound::Bashing,
			Wound::Aggravated => Wound::Lethal,
		}
	}

	#[must_use]
	pub fn poke(self) -> Wound {
		if let Wound::Aggravated = self {
			Wound::None
		} else {
			self.upgrade()
		}
	}

	pub fn poke_mut(&mut self) {
		*self = self.poke();
	}
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Damage {
	#[serde(skip_serializing_if = "is_zero")]
	aggravated: u8,
	#[serde(skip_serializing_if = "is_zero")]
	lethal: u8,
	#[serde(skip_serializing_if = "is_zero")]
	bashing: u8,
}

impl Damage {
	#[must_use]
	pub fn new(bashing: u8, lethal: u8, aggravated: u8) -> Self {
		Self {
			aggravated,
			lethal,
			bashing,
		}
	}

	#[must_use]
	pub fn get(&self, wound: Wound) -> u8 {
		match wound {
			Wound::None => 0,
			Wound::Bashing => self.bashing,
			Wound::Lethal => self.lethal,
			Wound::Aggravated => self.aggravated,
		}
	}

	#[must_use]
	pub fn get_mut(&mut self, wound: Wound) -> Option<&mut u8> {
		match wound {
			Wound::None => None,
			Wound::Bashing => Some(&mut self.bashing),
			Wound::Lethal => Some(&mut self.lethal),
			Wound::Aggravated => Some(&mut self.aggravated),
		}
	}

	#[must_use]
	pub fn total(&self) -> u8 {
		self.bashing + self.lethal + self.aggravated
	}

	pub fn poke(&mut self, wound: Wound) -> Wound {
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
		wound.poke()
	}

	pub fn poke_index(&mut self, i: u8) -> Wound {
		self.poke(self[i])
	}
}

impl Index<u8> for Damage {
	type Output = Wound;

	fn index(&self, i: u8) -> &Self::Output {
		if i < self.aggravated {
			&Wound::Aggravated
		} else if i >= self.aggravated && i < (self.aggravated + self.lethal) {
			&Wound::Lethal
		} else if i >= (self.aggravated + self.lethal) && i < self.total() {
			&Wound::Bashing
		} else {
			&Wound::None
		}
	}
}

use std::ops::Index;

use serde::{Deserialize, Serialize};

use crate::util::is_zero;

#[derive(Default, Debug, Clone)]
pub enum Wound {
	#[default]
	None,
	Bashing,
	Lethal,
	Aggravated,
}

impl Wound {
	#[must_use]
	pub fn inc(&self) -> Wound {
		match self {
			Wound::None => Wound::Bashing,
			Wound::Bashing => Wound::Lethal,
			Wound::Lethal => Wound::Aggravated,
			Wound::Aggravated => Wound::Aggravated,
		}
	}

	#[must_use]
	pub fn poke(&self) -> Wound {
		if let Wound::Aggravated = self {
			Wound::None
		} else {
			self.inc()
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
	aggravated: u16,
	#[serde(skip_serializing_if = "is_zero")]
	lethal: u16,
	#[serde(skip_serializing_if = "is_zero")]
	bashing: u16,
}

impl Damage {
	pub fn new(bashing: u16, lethal: u16, aggravated: u16) -> Self {
		Self {
			aggravated,
			lethal,
			bashing,
		}
	}

	pub fn get(&self, wound: &Wound) -> u16 {
		match wound {
			Wound::None => 0,
			Wound::Bashing => self.bashing,
			Wound::Lethal => self.lethal,
			Wound::Aggravated => self.aggravated,
		}
	}

	pub fn sum(&self) -> u16 {
		self.bashing + self.lethal + self.aggravated
	}

	pub fn dec(&mut self, wound: &Wound) {
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

	pub fn inc(&mut self, wound: &Wound) {
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

impl Index<u16> for Damage {
	type Output = Wound;

	fn index(&self, i: u16) -> &Self::Output {
		if i < self.aggravated {
			&Wound::Aggravated
		} else if i >= self.aggravated && i < (self.aggravated + self.lethal) {
			&Wound::Lethal
		} else if i >= (self.aggravated + self.lethal)
			&& i < (self.aggravated + self.lethal + self.bashing)
		{
			&Wound::Bashing
		} else {
			&Wound::None
		}
	}
}

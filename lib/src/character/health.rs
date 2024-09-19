use carboxyl::{lift, Sink};
use serde::{Deserialize, Serialize};

use crate::{
	is_zero,
	reactive::{RxTrait, RxAttributes},
};

#[derive(Clone, Debug, Serialize)]
pub struct Health {
	pub track: Damage,
	pub max_health: RxTrait,
	pub wound_penalty: RxTrait,
}

impl Health {
	pub fn new(attributes: &RxAttributes, size: &RxTrait) -> Self {
		let max_health = size + &attributes.stamina;

		let health = Sink::new().stream().hold(Damage::default());

		let wound_penalty = RxTrait::from(lift!(
			|mh, health| {
				match mh - health.sum().min(mh) {
					2 => 1,
					1 => 2,
					0 => 3,
					_ => 0,
				}
			},
			&max_health.signal(),
			&health
		));

		Self {
			track: Damage::default(),
			max_health,
			wound_penalty,
		}
	}
}

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
	aggravated: u8,
	#[serde(skip_serializing_if = "is_zero")]
	lethal: u8,
	#[serde(skip_serializing_if = "is_zero")]
	bashing: u8,
}

impl Damage {
	pub fn new(bashing: u8, lethal: u8, aggravated: u8) -> Self {
		Self {
			aggravated,
			lethal,
			bashing,
		}
	}

	pub fn get(&self, wound: &Wound) -> u8 {
		match wound {
			Wound::None => 0,
			Wound::Bashing => self.bashing,
			Wound::Lethal => self.lethal,
			Wound::Aggravated => self.aggravated,
		}
	}

	pub fn get_i(&self, i: usize) -> Wound {
		// println!("{i}");
		if i < self.aggravated as usize {
			Wound::Aggravated
		} else if i >= self.aggravated as usize && i < (self.aggravated + self.lethal) as usize {
			Wound::Lethal
		} else if i >= (self.aggravated + self.lethal) as usize
			&& i < (self.aggravated + self.lethal + self.bashing) as usize
		{
			Wound::Bashing
		} else {
			Wound::None
		}
	}

	pub fn sum(&self) -> u8 {
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

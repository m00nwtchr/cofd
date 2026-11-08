use std::ops::Index;

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

#[derive(Default, Debug, Clone, facet::Facet, PartialEq, Eq)]
// #[serde(default)]
pub struct Damage {
	// #[serde(skip_serializing_if = "is_zero")]
	aggravated: u8,
	// #[serde(skip_serializing_if = "is_zero")]
	lethal: u8,
	// #[serde(skip_serializing_if = "is_zero")]
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
		self.bashing
			.saturating_add(self.lethal)
			.saturating_add(self.aggravated)
	}

	pub fn poke(&mut self, wound: Wound) -> Wound {
		match wound {
			Wound::None => self.bashing = self.bashing.saturating_add(1),
			Wound::Bashing => {
				if self.bashing > 0 {
					self.bashing -= 1;
				}
				self.lethal = self.lethal.saturating_add(1);
			}
			Wound::Lethal => {
				if self.lethal > 0 {
					self.lethal -= 1;
				}
				self.aggravated = self.aggravated.saturating_add(1);
			}
			Wound::Aggravated => {
				if self.aggravated > 0 {
					self.aggravated -= 1;
				}
			}
		}
		wound.poke()
	}

	/// Applies damage at the requested severity, upgrading existing damage as
	/// required by the CofD damage track. Returns `false` without changing the
	/// track when the requested damage cannot fit.
	pub fn apply(&mut self, wound: Wound, points: u8, capacity: u8) -> bool {
		if matches!(wound, Wound::None) {
			return points == 0;
		}

		let mut next = self.clone();
		for _ in 0..points {
			if next.total() < capacity {
				match wound {
					Wound::Bashing => next.bashing = next.bashing.saturating_add(1),
					Wound::Lethal => next.lethal = next.lethal.saturating_add(1),
					Wound::Aggravated => next.aggravated = next.aggravated.saturating_add(1),
					Wound::None => unreachable!(),
				}
				continue;
			}

			let upgraded = match wound {
				Wound::Bashing | Wound::Lethal => {
					if next.bashing > 0 {
						next.bashing -= 1;
						next.lethal = next.lethal.saturating_add(1);
						true
					} else if next.lethal > 0 {
						next.lethal -= 1;
						next.aggravated = next.aggravated.saturating_add(1);
						true
					} else {
						false
					}
				}
				Wound::Aggravated => {
					if next.lethal > 0 {
						next.lethal -= 1;
						next.aggravated = next.aggravated.saturating_add(1);
						true
					} else if next.bashing > 0 {
						next.bashing -= 1;
						next.aggravated = next.aggravated.saturating_add(1);
						true
					} else {
						false
					}
				}
				Wound::None => false,
			};
			if !upgraded {
				return false;
			}
		}
		*self = next;
		true
	}

	/// Heals damage at the requested severity, downgrading a wound when the
	/// severity is lethal or aggravated.
	pub fn heal(&mut self, wound: Wound, points: u8) {
		for _ in 0..points {
			match wound {
				Wound::None => {}
				Wound::Bashing => self.bashing = self.bashing.saturating_sub(1),
				Wound::Lethal => {
					if self.lethal > 0 {
						self.lethal -= 1;
						self.bashing = self.bashing.saturating_add(1);
					}
				}
				Wound::Aggravated => {
					if self.aggravated > 0 {
						self.aggravated -= 1;
						self.lethal = self.lethal.saturating_add(1);
					}
				}
			}
		}
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
		} else if i >= self.aggravated && i < self.aggravated.saturating_add(self.lethal) {
			&Wound::Lethal
		} else if i >= self.aggravated.saturating_add(self.lethal) && i < self.total() {
			&Wound::Bashing
		} else {
			&Wound::None
		}
	}
}

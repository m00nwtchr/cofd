use serde::{Deserialize, Serialize};

use super::{Merit, SplatTrait, XSplat, YSplat, ZSplat};

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct Mortal;

impl SplatTrait for Mortal {
	fn set_xsplat(&mut self, _splat: Option<XSplat>) {}

	fn set_ysplat(&mut self, _splat: Option<YSplat>) {}

	fn set_zsplat(&mut self, _splat: Option<ZSplat>) {}

	fn xsplats(&self) -> Vec<XSplat> {
		Vec::new()
	}

	fn ysplats(&self) -> Vec<YSplat> {
		Vec::new()
	}

	fn zsplats(&self) -> Vec<ZSplat> {
		Vec::new()
	}

	fn merits(&self) -> Vec<Merit> {
		Merit::all()
	}
}

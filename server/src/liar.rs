//! Module to represent the pure side of liar's dice.
//! With no dependency on networking and such.

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Bid {
	quantity: u8,
	face: Face,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(into = "u64")]
#[serde(try_from = "u64")]
pub enum Face {
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
}

impl From<Face> for u64 {
	fn from(f: Face) -> Self {
		match f {
			Face::One => 1,
			Face::Two => 2,
			Face::Three => 3,
			Face::Four => 4,
			Face::Five => 5,
			Face::Six => 6,
		}
	}
}

#[derive(Debug)]
pub struct NotAFace;

impl Display for NotAFace {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "A dice's face can only be from 1 to 6.")
	}
}

impl TryFrom<u64> for Face {
	type Error = NotAFace;

	fn try_from(value: u64) -> Result<Self, Self::Error> {
		match value {
			1 => Ok(Face::One),
			2 => Ok(Face::Two),
			3 => Ok(Face::Three),
			4 => Ok(Face::Four),
			5 => Ok(Face::Five),
			6 => Ok(Face::Six),
			_ => Err(NotAFace),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BidVariation {
	/// Higher quantity of any face, or same quantity of higher face.
	RaiseQuantity,
	/// Higher quantity on same face, or any quantity on higher face.
	RaiseFace,
	/// Higher quantity on same face, or same quantity of higher face.
	Orthogonal,
}

pub fn is_bid_valid(current: Bid, raise: Bid, variation: BidVariation) -> bool {
	use BidVariation::*;
	match variation {
		RaiseQuantity => {
			raise.quantity > current.quantity
				|| (raise.face > current.face && raise.quantity == current.quantity)
		}
		RaiseFace => {
			raise.face > current.face
				|| (raise.quantity > current.quantity && raise.face == current.face)
		}
		Orthogonal => {
			(raise.face > current.face && raise.quantity == current.quantity)
				|| (raise.face == current.face && raise.quantity > current.quantity)
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Settings {
	pub wild_ones: bool,
	pub bid_variation: BidVariation,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			wild_ones: false,
			bid_variation: BidVariation::RaiseFace,
		}
	}
}

pub fn challenge_bid(bid: Bid, pool: impl IntoIterator<Item = Face>, settings: Settings) -> bool {
	pool.into_iter()
		.filter(|&f| f == bid.face || (settings.wild_ones && f == Face::One))
		.count() >= bid.quantity as usize
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn higher_quantity_bid_validation() {
		use BidVariation::*;
		let current = Bid {
			quantity: 5,
			face: Face::Three,
		};
		let raise = Bid {
			quantity: 6,
			face: Face::Two,
		};
		assert!(is_bid_valid(current, raise, RaiseQuantity));
		let raise = Bid {
			quantity: 5,
			face: Face::Four,
		};
		assert!(is_bid_valid(current, raise, RaiseQuantity));
		let raise = Bid {
			quantity: 5,
			face: Face::Three,
		};
		assert!(!is_bid_valid(current, raise, RaiseQuantity));
	}

	#[test]
	fn higher_face_bid_validation() {
		use BidVariation::*;
		let current = Bid {
			quantity: 5,
			face: Face::Three,
		};
		let raise = Bid {
			quantity: 2,
			face: Face::Four,
		};
		assert!(is_bid_valid(current, raise, RaiseFace));
		let raise = Bid {
			quantity: 6,
			face: Face::Three,
		};
		assert!(is_bid_valid(current, raise, RaiseFace));
		let raise = Bid {
			quantity: 5,
			face: Face::Two,
		};
		assert!(!is_bid_valid(current, raise, RaiseFace));
	}

	#[test]
	fn orthogonal_bid_validation() {
		use BidVariation::*;
		let current = Bid {
			quantity: 5,
			face: Face::Three,
		};
		let raise = Bid {
			quantity: 7,
			face: Face::Three,
		};
		assert!(is_bid_valid(current, raise, Orthogonal));
		let raise = Bid {
			quantity: 5,
			face: Face::Five,
		};
		assert!(is_bid_valid(current, raise, Orthogonal));
		let raise = Bid {
			quantity: 6,
			face: Face::Six,
		};
		assert!(!is_bid_valid(current, raise, Orthogonal));
	}
}

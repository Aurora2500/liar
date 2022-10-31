use core::fmt;

use actix::{Actor, Addr, Handler, Message as AMessage, StreamHandler};
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};
use serde::{Deserialize, Serialize};
use slotmap::DefaultKey;

use serde_json::from_reader;

use crate::liar;

use super::{Game, SetBid};

/// Represents a player.
pub struct Player {
	/// The nickname of the player.
	pub(super) name: String,
	pub(super) key: DefaultKey,
	pub(super) game: Addr<Game>,
}

impl fmt::Debug for Player {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		f.debug_tuple("Player").field(&self.name).finish()
	}
}

impl Player {
	pub fn new(game: Addr<Game>) -> Self {
		Self {
			game,
			name: "Guest".into(),
			key: Default::default(),
		}
	}
}

impl Actor for Player {
	type Context = WebsocketContext<Self>;
}

pub struct NewKey(pub DefaultKey);

impl AMessage for NewKey {
	type Result = ();
}

impl Handler<NewKey> for Player {
	type Result = ();

	fn handle(&mut self, msg: NewKey, ctx: &mut Self::Context) -> Self::Result {
		self.key = msg.0
	}
}

impl StreamHandler<Result<Message, ProtocolError>> for Player {
	fn handle(&mut self, item: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
		let action = if let Ok(Message::Text(t)) = item {
			let res = from_reader::<&[u8], PlayerAction>(t.as_ref());
			match res {
				Ok(v) => v,
				Err(why) => {
					println!("Error deserializing: {:#?}", why);
					return;
				}
			}
		} else {
			return;
		};
		println!("---- Player action ----\n{:#?}\n", action);
		match action {
			PlayerAction::SetName{name} => {
				self.name = name;
			},
			PlayerAction::PlaceBid{bid} => {
				let req = self.game.send(SetBid(bid, self.key));
			},
			PlayerAction::Challenge => todo!(),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all="snake_case")]
enum PlayerAction {
	SetName{name: String},
	PlaceBid{bid: liar::Bid},
	Challenge,
}

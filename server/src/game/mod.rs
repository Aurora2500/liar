use crate::liar::*;

use actix::{Actor, ActorContext, Addr, Context, Handler, Message};
use serde::Serialize;
use slotmap::{DefaultKey, SlotMap};

mod player;

pub use player::Player;


use self::player::NewKey;

enum GameState {
	Lobby,
	Game { bid: Option<Bid> },
}

#[derive(Debug, Default)]
pub struct Game {
	pub settings: Settings,
	order: Vec<DefaultKey>,
	players: SlotMap<DefaultKey, Addr<Player>>,
}

impl Actor for Game {
	type Context = Context<Self>;
}

struct Shutdown;

impl Message for Shutdown {
	type Result = ();
}

impl Handler<Shutdown> for Game {
	type Result = ();

	fn handle(&mut self, _msg: Shutdown, ctx: &mut Self::Context) -> Self::Result {
		ctx.stop()
	}
}

pub struct NewPlayer(pub Addr<Player>);

impl Message for NewPlayer {
	type Result = ();
}

impl Handler<NewPlayer> for Game {
	type Result = ();

	fn handle(&mut self, msg: NewPlayer, ctx: &mut Self::Context) -> Self::Result {
		let key = self.players.insert(msg.0.clone());
		self.order.push(key);
		msg.0.do_send(NewKey(key));
	}
}

#[derive(Clone, Copy)]
struct SetBid(Bid, DefaultKey);

impl Message for SetBid {
	// Did it fail.
	type Result = bool;
}

impl Handler<SetBid> for Game {
	type Result = bool;

	fn handle(&mut self, msg: SetBid, _ctx: &mut Self::Context) -> Self::Result {
		let key = msg.1;
		self.players.iter()
			.filter_map(|(k, v)| {
				if k == key {
					None
				} else {
					Some(v)
				}
			}).for_each(|player| {
				player.do_send(msg);
			});
		true
	}
}

impl Handler<SetBid> for Player {
	type Result = bool;

	fn handle(&mut self, msg: SetBid, ctx: &mut Self::Context) -> Self::Result {
		true
	}
}

struct SetName(String, DefaultKey);

impl Message for SetName {
	type Result = ();
}

impl Handler<SetName> for Game {
	type Result = ();

	fn handle(&mut self, msg: SetName, _ctx: &mut Self::Context) -> Self::Result {
		
	}
}
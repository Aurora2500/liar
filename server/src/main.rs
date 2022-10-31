use std::{collections::HashMap};

use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws::{WsResponseBuilder};
use game::{Game, NewPlayer, Player};
use rand::Rng;
use serde_json::json;
use tokio::{runtime::Builder, sync::RwLock};

mod game;
mod liar;

type GamesMap = RwLock<HashMap<String, Addr<Game>>>;

#[get("/")]
async fn hello() -> &'static str {
	"Hello!"
}

#[post("/new_game")]
async fn new_game(games: web::Data<GamesMap>) -> impl Responder {
	let uniform = rand::distributions::Uniform::new_inclusive('a', 'z');
	let mut games = games.write().await;
	let mut rng = rand::thread_rng();
	let code: String = loop {
		let code: String = (0..5).map(|_| rng.sample(uniform)).collect();
		if (games.get(&code)).is_none() {
			break code;
		}
	};

	let game = Game::start_default();

	games.insert(code.clone(), game);

	println!("There are {} games rn", games.len());

	let val = json!({
		"data": {
			"code": code
		}
	});
	web::Json(val)
}

#[get("/{code}")]
async fn ping(code: web::Path<String>) -> String {
	code.clone()
}

#[get("/{code}/ws")]
async fn game_ws(
	req: HttpRequest,
	stream: web::Payload,
	code: web::Path<String>,
	games: web::Data<GamesMap>,
) -> Option<Result<HttpResponse, actix_web::Error>> {
	let lock = games.read().await;
	if let Some(game) = lock.get(&*code) {
		println!("---------- New conn ----------");
		let res =
			WsResponseBuilder::new(Player::new(game.clone()), &req, stream)
			.start_with_addr();
		let (addr, res) = match res {
			Ok(p) => p,
			Err(why) => {
				return Some(Err(why));
			}
		};
		game.do_send(NewPlayer(addr));
		println!("{:#?}", res);
		Some(Ok(res))
	} else {
		None
	}
}

fn main() {
	let games: web::Data<GamesMap> = web::Data::new(GamesMap::default());

	let rt = Builder::new_current_thread()
		.enable_all()
		.build()
		.expect("Failed to build runtime");

	rt.block_on(async {
		HttpServer::new(move || {
			let cors = Cors::permissive();
			App::new()
			.wrap(cors)
				.app_data(games.clone())
				.service(hello)
				.service(new_game)
				.service(ping)
				.service(game_ws)
		})
		.bind(("127.0.0.1", 4000))
		.expect("Failed building server.")
		.run()
		.await
		.ok();
	})
}

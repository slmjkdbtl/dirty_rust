// wengwengweng

use std::time::Instant;

use dirty::codec::*;
use dirty::http;
use dirty::fs;
use dirty::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct CardSet {
	set_name: String,
	set_code: String,
	set_rarity: String,
	set_price: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardImage {
	id: String,
	image_url: String,
	image_url_small: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardPrice {
	cardmarket_price: Option<f32>,
	tcgplayer_price: Option<f32>,
	ebay_price: Option<f32>,
	amazon_price: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
	id: String,
	name: String,
	r#type: String,
	desc: String,
	race: String,
	archetype: Option<String>,
	card_images: Vec<CardImage>,
// 	card_prices: Option<Vec<CardPrice>>,
// 	card_sets: Option<Vec<CardSet>>,
}

const DB_URL: &'static str = "https://db.ygoprodeck.com/api/v5/cardinfo.php";

fn get() -> Result<()> {

	let res = http::get(DB_URL)?;
	let json = res.text();

	fs::write("examples/res/ygo.json", res.text())?;

	return Ok(());

}

fn main() -> Result<()> {

// 	get()?;

	let now = Instant::now();
	let data: Vec<Card> = json::decode(include_str!("res/ygo.json"))?;

	println!("{} cards", data.len());
	println!("{}", now.elapsed().as_secs_f32());

	return Ok(());

}


use std::error::Error;
use serde_json::Value;
use crate::{Pokemon, Sprites, Stats};

/*
Extracting a single function from the main source feels a bit stupid, but the json
matching was taking up too much space. Seems reasonable for now
 */

pub async fn get_poke(poke_id: String) -> std::result::Result<Pokemon, Box<dyn Error>> {
    let url = "https://absanthosh.github.io/PokedexData/PokemonData.json";

    let response = reqwest::get(url).await?;
    let json: Value = response.json().await?;

    let formatted_key = poke_id;

    match json.get(formatted_key) {
        Some(value) => {
            let poke  = Pokemon {
                name: value.get("Name").and_then(Value::as_str).unwrap_or("").to_string(),
                description: value.get("Description").and_then(Value::as_str).unwrap_or("").to_string(),
                stats: Stats {
                    hp: value.get("Stats").and_then(Value::as_object).and_then(|v| v.get("HP")).and_then(Value::as_u64).unwrap_or(0) as u8,
                    attack: value.get("Stats").and_then(Value::as_object).and_then(|v| v.get("Attack")).and_then(Value::as_u64).unwrap_or(0) as u8,
                    defense: value.get("Stats").and_then(Value::as_object).and_then(|v| v.get("Defense")).and_then(Value::as_u64).unwrap_or(0) as u8,
                    sp_attack: value.get("Stats").and_then(Value::as_object).and_then(|v| v.get("Sp. Attack")).and_then(Value::as_u64).unwrap_or(0) as u8,
                    sp_defense: value.get("Stats").and_then(Value::as_object).and_then(|v| v.get("Sp. Defense")).and_then(Value::as_u64).unwrap_or(0) as u8,
                    speed: value.get("Stats").and_then(Value::as_object).and_then(|v| v.get("Speed")).and_then(Value::as_u64).unwrap_or(0) as u8,
                },
                types: value.get("Types").and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_str).map(String::from).collect()).unwrap_or_else(Vec::new),
                ability: value.get("Ability").and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_str).map(String::from).collect()).unwrap_or_else(Vec::new),
                sprites: Sprites {
                    misc: value.get("Sprites").and_then(Value::as_object).and_then(|v| v.get("Misc")).and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_str).map(String::from).collect()).unwrap_or_else(Vec::new),
                    no_gen_normal: value.get("Sprites").and_then(Value::as_object).and_then(|v| v.get("NoGenNormal")).and_then(Value::as_str).unwrap_or("").to_string(),
                    no_gen_shiny: value.get("Sprites").and_then(Value::as_object).and_then(|v| v.get("NoGenShiny")).and_then(Value::as_str).unwrap_or("").to_string(),
                },
                evolution_chain: value.get("EvolutionChain").and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_str).map(String::from).collect()).unwrap_or_else(Vec::new),
            };
            Ok(poke)
        },
        None => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Pokemon data not found",
        ))),
    }
}

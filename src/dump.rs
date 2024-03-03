
use reqwest::Error;
use serde_json::{Value, Error as JsonError};
use serde::Deserialize;
use std::fmt;

// impl fmt::Debug for Pokemon {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Pokemon {{\n\tName: {},\
//         \n\tDescription: {},\
//         \n\tStats: {{\n\t\thp: {},\n\t\tattack: {},\n\t\tdefense: {},\n\t\tsp_attack: {},\n\t\tsp_defense: {},\n\t\tspeed: {}\n\t}},\
//         \n\tTypes: {:?},\
//         \n\tAbility: {:?},\
//         \n\tSprites: {:?},\
//         \n\tEvolution Chain: {:?}\
//         \n}}",
//                self.name, self.description, self.stats.hp, self.stats.attack, self.stats.defense, self.stats.sp_attack, self.stats.sp_defense, self.stats.speed,
//                self.types, self.ability, self.sprites, self.evolution_chain)
//     }
// }

impl fmt::Debug for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pokemon {{\n\tName: {},\
        \n\tDescription: {},\
        \n\tStats: {{\n\t\thp: {},\n\t\tattack: {},\n\t\tdefense: {},\n\t\tsp_attack: {},\n\t\tsp_defense: {},\n\t\tspeed: {}\n\t}},\
        \n\tTypes: {:?},\
        \n\tAbility: {:?},\
        \n\tSprites: {{\n\t\tMisc: {:?},\n\t\tNoGenNormal: {},\n\t\tNoGenShiny: {}\n\t}},\
        \n\tEvolution Chain: {:?}\
        \n}}",
               self.name, self.description, self.stats.hp, self.stats.attack, self.stats.defense, self.stats.sp_attack, self.stats.sp_defense, self.stats.speed,
               self.types, self.ability, self.sprites.misc, self.sprites.no_gen_normal, self.sprites.no_gen_shiny, self.evolution_chain)
    }
}

#[derive(Deserialize)]
struct Pokemon {
    name: String,
    description: String,
    stats: Stats,
    types: Vec<String>,
    ability: Vec<String>,
    sprites: Sprites,
    evolution_chain: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Stats {
    hp: u8,
    attack: u8,
    defense: u8,
    sp_attack: u8,
    sp_defense: u8,
    speed: u8,
}

#[derive(Deserialize, Debug)]
struct Sprites {
    misc: Vec<String>,
    no_gen_normal: String,
    no_gen_shiny: String,
}

async fn get_poke(poke_id: String) -> Result<Pokemon, Box<dyn std::error::Error>> {
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

#[tokio::main]
async fn main() {
    let mut poke_id = String::new();
    println!("Enter the Pokemon ID:");
    std::io::stdin().read_line(&mut poke_id).expect("Failed to read line");
    poke_id = poke_id.trim().to_string();
    match get_poke(poke_id).await {
        Ok(poke) => println!("Pokemon data: {:?}", poke),
        Err(e) => println!("Error: {}", e),
    }
}
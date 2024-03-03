// ANCHOR: imports
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::fs;
use std::io::{stdout, Result};
use std::rc::Rc;
use serde_json::{Value, Error as JsonError};
use serde::Deserialize;
use std::fmt;
use std::fmt::format;
// ANCHOR_END: imports

// ANCHOR: setup

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

async fn get_poke(poke_id: String) -> std::result::Result<Pokemon, Box<dyn std::error::Error>> {
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
async fn main() -> Result<()> {

    let mut poke_id = String::new();
    println!("Enter the Pokemon ID:");
    std::io::stdin().read_line(&mut poke_id).expect("Failed to read line");
    poke_id = poke_id.trim().to_string();

    let poke_result = get_poke(poke_id).await;

    let poke: Pokemon = match poke_result {
        Ok(poke_data) => poke_data,
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // ANCHOR_END: setup

    //ANCHOR: draw

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let dir;
            let dim;
            if(area.height*3 < area.width) {
                dir = Direction::Horizontal;
                dim = area.height;
            }else{
                dir = Direction::Vertical;
                dim = area.width;
            }

            let outer = Layout::default()
                .direction(dir)
                .constraints(vec![
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                    Constraint::Min(0),
                ])
                .split(area);

            let image_block = Paragraph::new(format!("{}", poke.description))
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .title(format!("{}", poke.name))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                );
            let text_block = Paragraph::new(format!(
                "HP: {}\nAttack: {}\nDefense: {}\nSp. Attack: {}\nSp. Defense: {}\nSpeed: {}",
                poke.stats.hp,
                poke.stats.attack,
                poke.stats.defense,
                poke.stats.sp_attack,
                poke.stats.sp_defense,
                poke.stats.speed
            ))
                .style(Style::default().fg(Color::Magenta))
                .block(
                    Block::default()
                        .title("Stats")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                );
            frame.render_widget(image_block, outer[0]);
            frame.render_widget(text_block, outer[1]);
        })?;
        //ANCHOR_END: draw

        // ANCHOR: handle-events
        if event::poll(std::time::Duration::from_millis(20))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        _ => continue,
                    }
                }
            }
        }
        // ANCHOR_END: handle-events
    }


    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
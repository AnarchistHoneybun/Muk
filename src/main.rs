use std::error::Error;
// ANCHOR: imports
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
// use crossterm::style::Color;
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

fn get_color_for_type(pokemon_type: &str) -> Color {
    match pokemon_type {
        "Fire" => Color::Red,
        "Water" => Color::Blue,
        "Grass" => Color::Green,
        "Electric" => Color::Yellow,
        "Psychic" => Color::Magenta,
        "Poison" => Color::Magenta,
        "Ice" => Color::Cyan,
        "Dragon" => Color::LightRed,
        "Dark" => Color::Black,
        "Fairy" => Color::White,
        _ => Color::White, // Default color for unknown types
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
    poke_id = poke_id.trim().to_string();

    let mut poke_result: std::result::Result<Pokemon, Box<dyn Error>>;

    let mut poke: Pokemon;
    // assign default values to the poke variable
    poke = Pokemon {
        name: "Muk".to_string(),
        description: "Welcome to the pokédex, your personal database of all pokemon you encounter in your travels".to_string(),
        stats: Stats {
            hp: 0,
            attack: 0,
            defense: 0,
            sp_attack: 0,
            sp_defense: 0,
            speed: 0,
        },
        types: vec!["Poison".to_string(), "Grass".to_string()],
        ability: vec!["Overgrow".to_string(), "Chlorophyll".to_string()],
        sprites: Sprites {
            misc: vec!["".to_string()],
            no_gen_normal: "https://absanthosh.github.io/PokedexData/Sprites/001MS.png".to_string(),
            no_gen_shiny: "https://absanthosh.github.io/PokedexData/Sprites/001MS.png".to_string(),
        },
        evolution_chain: vec!["".to_string()],
    };


    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // create a mutable string to use for the search field
    let mut search_string: String = String::new();
    search_string = "".to_string();

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

            let pokedex = Layout::default()
                .direction(dir)
                .constraints(vec![
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                    Constraint::Min(0),
                ])
                .split(area);

            let d_pad = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ])
                .split(pokedex[1]);

            let image_block = Paragraph::new(format!("{}", poke.description))
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(get_color_for_type(&poke.types[0])))
                .block(
                    Block::default()
                        .title(format!("{}", poke.name))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                );

            // define a string variable with only underscores
            // it should check the length of the search_string and contain 4-n underscores in it
            // where n is the length of the search_string

            let filler = "_".repeat(4 - search_string.len());


            let search_field = Paragraph::new(format!("{} {}", search_string, filler))
                .style(Style::default().fg(Color::Red))
                .block(
                    Block::default()
                        .title("Search")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                );

            let stat_left = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(80),
                    Constraint::Percentage(20),
                ])
                .split(d_pad[1]);
            let stat_left_top = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(6),
                    Constraint::Percentage(6),
                    Constraint::Percentage(6),
                    Constraint::Percentage(6),
                    Constraint::Percentage(6),
                    Constraint::Percentage(6),
                    Constraint::Percentage(64),
                ])
                .split(stat_left[0]);


            let hp_gauge = LineGauge::default()
                .gauge_style(Style::default().fg(Color::Green).bg(Color::DarkGray))
                .label("HP")
                .line_set(symbols::line::THICK)
                .ratio((poke.stats.hp as f64 / 100.0).min(1.0));
            frame.render_widget(hp_gauge, stat_left_top[0]);
            let attack_gauge = LineGauge::default()
                .gauge_style(Style::default().fg(Color::Red).bg(Color::DarkGray))
                .label("AT")
                .line_set(symbols::line::THICK)
                .ratio((poke.stats.attack as f64 / 100.0).min(1.0));
            frame.render_widget(attack_gauge, stat_left_top[1]);
            let def_gauge = LineGauge::default()
                .gauge_style(Style::default().fg(Color::Yellow).bg(Color::DarkGray))
                .label("DF")
                .line_set(symbols::line::THICK)
                .ratio((poke.stats.defense as f64 / 100.0).min(1.0));
            frame.render_widget(def_gauge, stat_left_top[2]);
            let sp_attack_gauge = LineGauge::default()
                .gauge_style(Style::default().fg(Color::Magenta).bg(Color::DarkGray))
                .label("SA")
                .line_set(symbols::line::THICK)
                .ratio((poke.stats.sp_attack as f64 / 100.0).min(1.0));
            frame.render_widget(sp_attack_gauge, stat_left_top[3]);
            let sp_defense_gauge = LineGauge::default()
                .gauge_style(Style::default().fg(Color::LightBlue).bg(Color::DarkGray))
                .label("SD")
                .line_set(symbols::line::THICK)
                .ratio((poke.stats.sp_defense as f64 / 100.0).min(1.0));
            frame.render_widget(sp_defense_gauge, stat_left_top[4]);
            let speed_gauge = LineGauge::default()
                .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
                .label("SP")
                .line_set(symbols::line::THICK)
                .ratio((poke.stats.speed as f64 / 100.0).min(1.0));
            frame.render_widget(speed_gauge, stat_left_top[5]);

            // let text_block = Paragraph::new(format!(
            //     "HP: {}\nAttack: {}\nDefense: {}\nSp. Attack: {}\nSp. Defense: {}\nSpeed: {}",
            //     poke.stats.hp,
            //     poke.stats.attack,
            //     poke.stats.defense,
            //     poke.stats.sp_attack,
            //     poke.stats.sp_defense,
            //     poke.stats.speed
            // ))
            //     .style(Style::default().fg(Color::White))
            //     .block(
            //         Block::default()
            //             .title("Stats")
            //             .borders(Borders::ALL)
            //             .border_type(BorderType::Rounded)
            //     );


            frame.render_widget(image_block, pokedex[0]);
            // frame.render_widget(text_block, d_pad[1]);
            frame.render_widget(search_field, d_pad[0]);
        })?;
        //ANCHOR_END: draw

        // ANCHOR: handle-events
        if event::poll(std::time::Duration::from_millis(20))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(x) => search_string.push(x),
                        KeyCode::Backspace => {
                            search_string.pop();
                            ()
                        },
                        KeyCode::Enter => {
                            poke_result = get_poke(search_string.clone()).await;
                            search_string.clear();

                            poke = match poke_result {
                                Ok(poke_data) => poke_data,
                                Err(e) => {
                                    println!("Error: {}", e);
                                    return Ok(());
                                }
                            };
                        },
                        KeyCode::Esc => break,
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
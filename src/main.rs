// ANCHOR: imports
use std::{fs::File, io::{copy, Cursor}};
use std::error::Error;
use std::io::{Result, stdout};
use std::io::Read;
use std::rc::Rc;

use anyhow;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rascii_art::{render_to, RenderOptions};
use rascii_art::charsets::*;
use ratatui::{prelude::*, widgets::*};
use ratatui::layout::Direction::Horizontal;
use ratatui::layout::Direction::Vertical;
use reqwest;
use serde::Deserialize;

mod color_lib;
mod api_call;


// ANCHOR_END: imports

// ANCHOR: structs


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

// ANCHOR_END: structs

async fn download_image_to(url: &str, file_name: &str) -> anyhow::Result<()> {
    // Send an HTTP GET request to the URL
    let response = reqwest::get(url).await?;
    // Create a new file to write the downloaded image to
    let mut file = File::create(file_name)?;

    // Create a cursor that wraps the response body
    let mut content =  Cursor::new(response.bytes().await?);
    // Copy the content from the cursor to the file
    copy(&mut content, &mut file)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {

    let mut image_url = "https://img.pokemondb.net/sprites/black-white/normal/muk.png";
    let file_name = "src/display_image/display.png";
    match download_image_to(image_url, file_name).await {
        Ok(_) => (),
        Err(e) => println!("error while downloading image: {}", e),
    }



    let mut poke_id = String::new();
    poke_id = poke_id.trim().to_string();

    let mut poke_result: std::result::Result<Pokemon, Box<dyn Error>>;

    let mut poke: Pokemon;
    // assign default values to the poke variable
    poke = Pokemon {
        name: "Welcome to the pokédex, your personal database of all pokemon you encounter in your travels".to_string(),
        description: "Enter to Search".to_string(),
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

    // create a mutable string to use for the search field
    let mut search_string: String = String::new();
    search_string = "".to_string();


    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;



    //ANCHOR: draw

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let dir;
            let dim;
            if area.height*3 < area.width {
                dir = Horizontal;
                dim = 50;
            }else{
                dir = Vertical;
                dim = 60;
            }

            let pokedex = Layout::default()
                .direction(dir)
                .constraints(vec![
                    Constraint::Percentage(dim),
                    Constraint::Percentage(100-dim),
                    Constraint::Min(0),
                ])
                .split(area);

            let screen = Layout::default()
                .direction(Vertical)
                .constraints(vec![
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                    Constraint::Min(0),
                ])
                .split(pokedex[0]);

            let d_pad = Layout::default()
                .direction(Vertical)
                .constraints(vec![
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ])
                .split(pokedex[1]);

            let image_header = Paragraph::new(format!("{}", poke.name.to_ascii_uppercase()))
                .wrap(Wrap { trim: true })
                .alignment(Alignment::Center)
                .style(Style::default()
                    .fg(color_lib::get_color_for_type(&poke.types[0])))
                .block(
                    Block::default()
                        .title("Pokédex")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                );

            let mut buffer = String::new();
            generate_graphic(&screen, &mut buffer);

            let screen_image = Paragraph::new(format!("{}", buffer))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true })
                .style(Style::default());

            let image_footer = Paragraph::new(format!("{}", poke.description))
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(color_lib::get_color_for_type(&poke.types[0])))
                .block(
                    Block::default()
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
                .direction(Horizontal)
                .constraints(vec![
                    Constraint::Percentage(80),
                    Constraint::Percentage(20),
                ])
                .split(d_pad[1]);
            let stat_left_top = Layout::default()
                .direction(Vertical)
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
            let stat_left_bottom = Layout::default()
                .direction(Vertical)
                .constraints(vec![
                    Constraint::Percentage(30),
                    Constraint::Percentage(70),
                ])
                .split(stat_left_top[6]);
            let type_space = Layout::default()
                .direction(Horizontal)
                .constraints(vec![
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(stat_left_bottom[0]);

            if poke.types.len()==2 {
                let type_1 = Paragraph::new(format!("{}",poke.types[0]))
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(color_lib::get_color_for_type(&poke.types[0])))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                    );
                frame.render_widget(type_1, type_space[0]);
                let type_2 = Paragraph::new(format!("{}",poke.types[1]))
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(color_lib::get_color_for_type(&poke.types[1])))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                    );
                frame.render_widget(type_2, type_space[1]);
            }else {
                let type_1 = Paragraph::new(format!("{}",poke.types[0]))
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(color_lib::get_color_for_type(&poke.types[0])))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                    );
                frame.render_widget(type_1, type_space[0]);
            }



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


            frame.render_widget(image_header, screen[0]);
            frame.render_widget(screen_image, screen[1]);
            frame.render_widget(image_footer, screen[2]);
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

                            // check search string length
                            // if it is not equal to 4, pad it with zeroes
                            if search_string.len() < 4 {
                                search_string = format!("{:0>4}", search_string);
                            }


                            poke_result = api_call::get_poke(search_string.clone()).await;
                            search_string.clear();

                            poke = match poke_result {
                                Ok(poke_data) => poke_data,
                                Err(e) => {
                                    println!("Error: {}", e);
                                    return Ok(());
                                }
                            };

                            image_url = &poke.sprites.misc[0];

                            match download_image_to(image_url, file_name).await {
                                Ok(_) => (),
                                Err(e) => println!("error while downloading image: {}", e),
                            }

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

fn generate_graphic(screen: &Rc<[Rect]>, mut buffer: &mut String) {
    render_to(
        "src/display_image/display.png",
        &mut buffer,
        &RenderOptions::new()
            .height(screen[1].height as u32)
            .charset(BLOCK)
    )
        .unwrap();
}

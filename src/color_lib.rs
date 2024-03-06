use ratatui::prelude::Color;

pub fn get_color_for_type(pokemon_type: &str) -> Color {
    match pokemon_type {
        "Fire" => Color::Rgb(255, 102, 0),
        "Water" => Color::Rgb(51, 204, 255),
        "Grass" => Color::Rgb(0, 153, 0),
        "Electric" => Color::Rgb(255, 255, 0),
        "Psychic" => Color::Rgb(102, 102, 153),
        "Poison" => Color::Rgb(102, 0, 204),
        "Ice" => Color::Rgb(102, 255, 255),
        "Dragon" => Color::Rgb(204, 0, 0),
        "Bug" => Color::Rgb(102, 102, 51),
        "Fighting" => Color::Rgb(153, 204, 255),
        "Flying" => Color::Rgb(255, 204, 0),
        "Ghost" => Color::Rgb(102, 0, 255),
        "Ground" => Color::Rgb(153, 102, 51),
        "Rock" => Color::Rgb(153, 102, 0),
        "Normal" => Color::Rgb(255, 255, 255),
        _ => Color::White, // Default color for unknown types
    }
}

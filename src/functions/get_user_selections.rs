use console::Style;
use dialoguer::{theme::ColorfulTheme, MultiSelect};

pub fn get_user_selections(options: &[&str], text: &str) -> Vec<String> {
    let selections = MultiSelect::with_theme(&custom_theme())
        .with_prompt(text)
        .items(options)
        .interact()
        .unwrap();

    let filtered_selections: Vec<String> = selections
        .into_iter()
        .filter(|&i| !options[i].contains("ISO"))
        .map(|i| options[i].to_string())
        .collect();

    filtered_selections
}

fn custom_theme() -> ColorfulTheme {
    ColorfulTheme {
        active_item_style: Style::new().fg(dialoguer::console::Color::Cyan).bold(),
        inactive_item_style: Style::new().fg(dialoguer::console::Color::White),
        ..Default::default()
    }
}

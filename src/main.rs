use std::{env, fs};
use toml::Value;

fn setup_directories(name: &str) {
    fs::create_dir_all(format!(
        "{home_dir}/Repos/colorgen-nvim/{name}/lua/{name}",
        home_dir = env::var("HOME").unwrap(),
    ))
    .expect("Unable to write dir");
}

fn generate_init(name: &str) {
    let init_data = format!(
        "local M = {{}}
local theme = require('{name}.theme')

M.setup = function()
  vim.cmd('hi clear')
  if vim.fn.exists('syntax_on') then
    vim.cmd('syntax reset')
  end

  vim.o.termguicolors = true
  vim.g.colors_name = '{name}'

  theme.set_highlights()
end

return M"
    );

    fs::write(
        format!(
            // TODO: use this for current dir to generate colorscheme env::current_dir()
            "{home_dir}/Repos/colorgen-nvim/{name}/lua/{name}/init.lua",
            home_dir = env::var("HOME").unwrap()
        ),
        init_data,
    )
    // TODO: handle error
    .expect("problem creating palette file");
}

fn generate_palette(template: &Value, name: &str) {
    let palette = &template.get("palette");

    if let Some(palette) = palette {
        let mut palette_data = String::from("local colors = {");

        for (key, val) in palette.as_table().unwrap().iter() {
            palette_data += format!("\n  {key} = {val},").as_str();
        }
        palette_data += "\n}";
        palette_data += "\n\nreturn";

        fs::write(
            format!(
                "{home_dir}/Repos/colorgen-nvim/{name}/lua/{name}/palette.lua",
                home_dir = env::var("HOME").unwrap()
            ),
            palette_data,
        )
        // TODO: handle error
        .expect("problem creating palette file");
    }
}

// hl(0, 'Normal', { fg = c.vscFront, bg = c.vscBack })
fn generate_colorscheme(value: &Value, colorscheme_data: &mut String) {
    if let Some(table) = value.as_table() {
        for (table_name, val) in table.iter() {
            if table_name != "palette" && table_name != "information" {
                // println!("table_name: {}", table_name);
                // println!("val: {}", val);

                dbg!(val);
                for (hl_group, hl_values) in val.as_table().unwrap().iter() {
                    if let Some(string) = hl_values.as_str() {
                        println!("string {string}");

                        let values = string.split(' ').collect::<Vec<&str>>();

                        // hl(0, 'Normal', { fg = c.vscFront, bg = c.vscBack })
                        // *colorscheme_data += format!("\n  {string}").as_str();
                        *colorscheme_data += format!(
                            "\n  hl(0, \"{hl_group}\", {{ fg = c.{fg}, bg = c.{bg} }})",
                            fg = values[0],
                            bg = values[1]
                        )
                        .as_str();
                    }
                }
            }
        }
    }
}

fn generate_theme(colorscheme_data: &str, name: &str) {
    let mut theme_data = format!(
        "
local c = require('{name}.colors')

local hl = vim.api.nvim_set_hl
local theme = {{}}

theme.set_highlights = function()
",
    );

    theme_data += colorscheme_data;

    theme_data += "\nend

return theme";

    fs::write(
        format!(
            "{home_dir}/Repos/colorgen-nvim/{name}/lua/{name}/theme.lua",
            home_dir = env::var("HOME").unwrap()
        ),
        theme_data,
    )
    // TODO: handle error
    .expect("problem creating theme file");
}

// TODO: look into preserve order

fn main() {
    let input = r#"
    [information]
      name = 'onedarker'
      background = 'dark'
      author = 'Christian Chiarulli <chrisatmachine@gmail.com>'

    [palette]
      fg = '#abb2bf'
      bg = '#1e222a'

      alt_fg = '#8b92a8'
      alt_bg = '#1b1f27'
      dark = '#1b1f27'
      accent = '#545862'
      popup_back = '#1e222a'
      search_orange = '#613214'
      line = '#282C34'


    [highlights]
      Normal = 'fg bg'
      SignColumn = 'fg bg'
      MsgArea = 'fg bg'
      ModeMsg = 'fg bg'
      MsgSeparator = 'fg bg'
      SpellBad = 'fg bg'
      SpellCap = 'fg bg'
      SpellLocal = 'fg bg'

    [LSP]
      Normal = 'alt_fg alt_bg'
      SignColumn = 'alt_fg alt_bg'
      MsgArea = 'alt_fg alt_bg'
      ModeMsg = 'alt_fg alt_bg'
      MsgSeparator = 'alt_fg alt_bg'
      SpellBad = 'alt_fg alt_bg'
      SpellCap = 'alt_fg alt_bg'
      SpellLocal = 'alt_fg alt_bg'
    "#;

    let template = input.parse::<Value>().unwrap();

    let name = template["information"]["name"].as_str().unwrap();

    let mut colorscheme_data = String::new();

    setup_directories(name);
    generate_init(name);
    generate_palette(&template, name);
    generate_colorscheme(&template, &mut colorscheme_data);
    generate_theme(&colorscheme_data, name);
}

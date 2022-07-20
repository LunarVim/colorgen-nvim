use std::fs;
use toml::Value;
mod args;
use args::ColorgenArgs;
use clap::Parser;
use regex::Regex;

fn setup_directories(name: &str) {
    fs::create_dir_all(format!("{name}/lua/{name}")).expect("Unable to write dir");
    fs::create_dir_all(format!("{name}/colors")).expect("Unable to write dir");
}

fn generate_init(name: &str, background: &str) {
    if background != "dark" && background != "light" {
        panic!("background must be set to dark or light");
    }

    let init_data = format!(
        "local M = {{}}
local theme = require('{name}.theme')

M.setup = function()
  vim.cmd('hi clear')

  vim.o.background = '{background}'
  if vim.fn.exists('syntax_on') then
    vim.cmd('syntax reset')
  end

  vim.o.termguicolors = true
  vim.g.colors_name = '{name}'

  theme.set_highlights()
end

return M"
    );

    fs::write(format!("{name}/lua/{name}/init.lua"), init_data)
        .expect("problem creating palette file");
}

fn generate_vim_colors_file(name: &str) {
    let vim_colors_file_data = format!(
        "lua << EOF
local {name} = require(\"{name}\")
{name}.setup({{}})
EOF"
    );

    fs::write(format!("{name}/colors/{name}.vim",), vim_colors_file_data)
        .expect("problem creating palette file");
}

fn generate_palette(template: &Value, name: &str) -> Vec<String> {
    let palette = &template.get("palette");

    let mut palette_keys: Vec<String> = Vec::new();

    if let Some(palette) = palette {
        let mut palette_data = String::from("local colors = {");

        for (key, val) in palette.as_table().expect("Value not a table").iter() {
            palette_keys.push(key.to_string());
            palette_data += format!("\n  {key} = {val},").as_str();
        }
        palette_data += "\n}";
        palette_data += "\n\nreturn colors";

        fs::write(format!("{name}/lua/{name}/palette.lua"), palette_data)
            .expect("problem creating palette file");
    }

    palette_keys
}

fn add_style_options(style: &str) -> String {
    let mut style_options = String::new();
    for option in style.chars() {
        match option {
            'o' => style_options += "standout=true, ",
            'u' => style_options += "underline=true, ",
            'c' => style_options += "undercurl=true, ",
            'd' => style_options += "underdouble=true, ",
            't' => style_options += "underdotted=true, ",
            'h' => style_options += "underdashed=true, ",
            's' => style_options += "strikethrough=true, ",
            'i' => style_options += "italic=true, ",
            'b' => style_options += "bold=true, ",
            'r' => style_options += "reverse=true, ",
            'n' => style_options += "nocombine=true, ",
            '-' => {}
            _ => panic!("invalid style option! {option}"),
        }
    }
    style_options.pop();
    style_options
}

fn parse_value(value: &str, palette_keys: &[String]) -> String {
    let re = Regex::new(r"^#([0-9a-f]{3}|[0-9a-f]{6}|[0-9A-F]{3}|[0-9A-F]{6})$")
        .expect("Invalid Expression");

    if value == "-" {
        "'NONE'".into()
    } else if re.is_match(value) {
        format!("'{value}'")
    } else if palette_keys.contains(&value.to_string()) {
        format!("c.{value}")
    } else {
        panic!("{value} is not a valid palette key");
    }
}

fn parse_blend(blend: &str) -> String {
    let blend = blend.parse::<i32>().expect("Could not parse int");

    if !(0..=100).contains(&blend) {
        panic!("blend must be between 0 and 100");
    }
    format!("{blend}")
}

fn write_line(value: &Value, colorscheme_data: &mut String, palette_keys: Vec<String>) {
    for (hl_group, hl_values) in value.as_table().expect("Value not a table").iter() {
        if let Some(string) = hl_values.as_str() {
            let values = string.split(' ').collect::<Vec<&str>>();

            match values[..] {
                // also handles link
                [fg] => {
                    if fg.contains("link:") {
                        *colorscheme_data += format!(
                            "\n  hl(0, \"{hl_group}\", {{ link = '{link}' }})",
                            link = fg.to_string().replace("link:", "")
                        )
                        .as_str();
                    } else {
                        *colorscheme_data += format!(
                            "\n  hl(0, \"{hl_group}\", {{ fg = {fg}, bg = 'NONE' }})",
                            fg = parse_value(fg, &palette_keys)
                        )
                        .as_str();
                    }
                }
                [fg, bg] => {
                    *colorscheme_data += format!(
                        "\n  hl(0, \"{hl_group}\", {{ fg = {fg}, bg = {bg} }})",
                        fg = parse_value(fg, &palette_keys),
                        bg = parse_value(bg, &palette_keys)
                    )
                    .as_str();
                }

                [fg, bg, style] => {
                    *colorscheme_data += format!(
                        "\n  hl(0, \"{hl_group}\", {{ fg = {fg}, bg = {bg}, {style_options} }})",
                        fg = parse_value(fg, &palette_keys),
                        bg = parse_value(bg, &palette_keys),
                        style_options = add_style_options(style)
                    )
                    .as_str();
                }

                [fg, bg, style, sp] => {
                    *colorscheme_data += format!(
                        "\n  hl(0, \"{hl_group}\", {{ fg = {fg}, bg = {bg}, sp = {sp}, {style_options} }})",
                        fg = parse_value(fg, &palette_keys),
                        bg = parse_value(bg, &palette_keys),
                        sp = parse_value(sp, &palette_keys),
                        style_options = add_style_options(style)
                    )
                    .as_str();
                }
                [fg, bg, style, sp, blend] => {
                    *colorscheme_data += format!(
                        "\n  hl(0, \"{hl_group}\", {{ fg = {fg}, bg = {bg}, sp = {sp}, blend={blend}, {style_options} }})",
                        fg = parse_value(fg, &palette_keys),
                        bg = parse_value(bg, &palette_keys),
                        sp = parse_value(sp, &palette_keys),
                        blend = parse_blend(blend),
                        style_options = add_style_options(style)
                    )
                    .as_str();
                }
                _ => {}
            }
        }
    }
}

fn generate_colorscheme(value: &Value, colorscheme_data: &mut String, palette_keys: &[String]) {
    if let Some(table) = value.as_table() {
        for (table_name, val) in table.iter() {
            if table_name != "palette" && table_name != "information" {
                *colorscheme_data += format!(
                    "\n
  -- {table_name}"
                )
                .as_str();
                write_line(val, colorscheme_data, palette_keys.to_vec());
            }
        }
    }
}

fn generate_theme(colorscheme_data: &str, name: &str) {
    let mut theme_data = format!(
        "
local c = require('{name}.palette')

local hl = vim.api.nvim_set_hl
local theme = {{}}

theme.set_highlights = function()",
    );

    theme_data += colorscheme_data;

    theme_data += "\nend

return theme";

    fs::write(format!("{name}/lua/{name}/theme.lua"), theme_data)
        .expect("problem creating theme file");
}

fn main() {
    let args: ColorgenArgs = ColorgenArgs::parse();

    let content = std::fs::read_to_string(args.filename).expect("Invalid filename");

    let template = content.parse::<Value>().expect("Invalid Toml");

    let name = template["information"]["name"]
        .as_str()
        .expect("Must contain an information table and name");

    let background = template["information"]["background"]
        .as_str()
        .expect("Must contain an information table and background");

    let mut colorscheme_data = String::new();

    setup_directories(name);
    generate_init(name, background);
    generate_vim_colors_file(name);
    let palette_keys = generate_palette(&template, name);
    generate_colorscheme(&template, &mut colorscheme_data, &palette_keys);
    generate_theme(&colorscheme_data, name);
}

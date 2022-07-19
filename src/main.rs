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

fn generate_palette(template: &Value, name: &str) {
    let palette = &template.get("palette");

    if let Some(palette) = palette {
        let mut palette_data = String::from("local colors = {");

        for (key, val) in palette.as_table().unwrap().iter() {
            palette_data += format!("\n  {key} = {val},").as_str();
        }
        palette_data += "\n}";
        palette_data += "\n\nreturn colors";

        fs::write(format!("{name}/lua/{name}/palette.lua"), palette_data)
            // TODO: handle error
            .expect("problem creating palette file");
    }
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

fn parse_value(value: &str) -> String {
    let re = Regex::new(r"^#([0-9a-f]{3}|[0-9a-f]{6}|[0-9A-F]{3}|[0-9A-F]{6})$")
        .expect("Invalid Expression");

    if value.contains("link:") {
        format!("'{}'", value.replace("link:", ""))
    } else if value == "-" {
        "'NONE'".into()
    } else if re.is_match(value) {
        format!("'{value}'")
    } else {
        format!("c.{value}")
    }
}

fn parse_blend(blend: &str) -> String {
    let blend = blend.parse::<i32>().unwrap();

    if blend > 100 || blend < 0 {
        panic!("blend must be between 0 and 100");
    }
    format!("{blend}")
}

fn write_line(value: &Value, colorscheme_data: &mut String) {
    for (hl_group, hl_values) in value.as_table().unwrap().iter() {
        if let Some(string) = hl_values.as_str() {
            let values = string.split(' ').collect::<Vec<&str>>();

            match values[..] {
                // also handles link
                [fg] => {
                    *colorscheme_data += format!(
                        "\n  hl(0, \"{hl_group}\", {{ fg = {fg}, bg = 'NONE' }})",
                        fg = parse_value(fg)
                    )
                    .as_str();
                }
                [fg, bg] => {
                    *colorscheme_data += format!(
                        "\n  hl(0, \"{hl_group}\", {{ fg = {fg}, bg = {bg} }})",
                        fg = parse_value(fg),
                        bg = parse_value(bg)
                    )
                    .as_str();
                }

                [fg, bg, style] => {
                    *colorscheme_data += format!(
                        "\n  hl(0, \"{hl_group}\", {{ fg = {fg}, bg = {bg}, {style_options} }})",
                        fg = parse_value(fg),
                        bg = parse_value(bg),
                        style_options = add_style_options(style)
                    )
                    .as_str();
                }

                [fg, bg, style, sp] => {
                    *colorscheme_data += format!(
                        "\n  hl(0, \"{hl_group}\", {{ fg = {fg}, bg = {bg}, sp = {sp}, {style_options} }})",
                        fg = parse_value(fg),
                        bg = parse_value(bg),
                        sp = parse_value(sp),
                        style_options = add_style_options(style)
                    )
                    .as_str();
                }
                [fg, bg, style, sp, blend] => {
                    *colorscheme_data += format!(
                        "\n  hl(0, \"{hl_group}\", {{ fg = {fg}, bg = {bg}, sp = {sp}, blend={blend}, {style_options} }})",
                        fg = parse_value(fg),
                        bg = parse_value(bg),
                        sp = parse_value(sp),
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

fn generate_colorscheme(value: &Value, colorscheme_data: &mut String) {
    if let Some(table) = value.as_table() {
        for (table_name, val) in table.iter() {
            if table_name != "palette" && table_name != "information" {
                *colorscheme_data += format!(
                    "\n
  -- {table_name}"
                )
                .as_str();
                write_line(val, colorscheme_data);
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

// TODO: look into preserve order
// TODO: save palette keys don't allow if not in that list
fn main() {
    let args: ColorgenArgs = ColorgenArgs::parse();

    let content = std::fs::read_to_string(args.filename).unwrap();

    let template = content.parse::<Value>().unwrap();

    let name = template["information"]["name"].as_str().unwrap();

    let background = template["information"]["background"].as_str().unwrap();

    let mut colorscheme_data = String::new();

    setup_directories(name);
    generate_init(name, background);
    generate_vim_colors_file(name);
    generate_palette(&template, name);
    generate_colorscheme(&template, &mut colorscheme_data);
    generate_theme(&colorscheme_data, name);
}

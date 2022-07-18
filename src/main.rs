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

    fs::write(format!("{name}/lua/{name}/init.lua"), init_data)
        // TODO: handle error
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
        palette_data += "\n\nreturn colors";

        fs::write(format!("{name}/lua/{name}/palette.lua"), palette_data)
            // TODO: handle error
            .expect("problem creating palette file");
    }
}

fn add_style_options(style: &str) -> String {
    let mut style_options = String::new();
    //TODO: let style_options = style.chars().map(|option| match option {...} ).join(", ")
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
    style_options
}

fn write_line(value: &Value, colorscheme_data: &mut String) {
    for (hl_group, hl_values) in value.as_table().unwrap().iter() {
        if let Some(string) = hl_values.as_str() {
            // TODO: I think you could refactor it like ```rust let fg = if let Some("-") = values.get(0) { "NONE" } else if let Some(fg) = values.get(0) { fg } else { "None" } ```
            //              • sp (or special): color name or "#RRGGBB"
            //              • blend: integer between 0 and 100
            //              • link: name of another highlight group to link
            // any time there is a - it is meant to be skipped or set to NONE
            let values = string.split(' ').collect::<Vec<&str>>();

            let re = Regex::new(r"^#([0-9a-f]{3}|[0-9a-f]{6}|[0-9A-F]{3}|[0-9A-F]{6})$").unwrap();

            match values[..] {
                [fg] => {
                    // TODO: break this out into a function
                    let fg = if fg == "-" {
                        "'NONE'".into()
                    } else if re.is_match(fg) {
                        format!("'{fg}'")
                    } else {
                        format!("c.{fg}")
                    };

                    *colorscheme_data +=
                        format!("\n  hl(0, \"{hl_group}\", {{ fg = {fg}, bg = 'NONE' }})",)
                            .as_str();
                }
                [fg, bg] => {
                    let fg = if fg == "-" {
                        "'NONE'".into()
                    } else if re.is_match(fg) {
                        format!("'{fg}'")
                    } else {
                        format!("c.{fg}")
                    };

                    let bg = if bg == "-" {
                        "'NONE'".into()
                    } else if re.is_match(bg) {
                        format!("'{bg}'")
                    } else {
                        format!("c.{bg}")
                    };

                    // TODO: std::fmt::Write; write!(string, "hello {variable}");
                    *colorscheme_data +=
                        format!("\n  hl(0, \"{hl_group}\", {{ fg = {fg}, bg = {bg} }})",).as_str();
                }

                [fg, bg, style] => {
                    let fg = if fg == "-" {
                        "'NONE'".into()
                    } else if re.is_match(fg) {
                        format!("'{fg}'")
                    } else {
                        format!("c.{fg}")
                    };

                    let bg = if bg == "-" {
                        "'NONE'".into()
                    } else if re.is_match(bg) {
                        format!("'{bg}'")
                    } else {
                        format!("c.{bg}")
                    };

                    // TODO: std::fmt::Write; write!(string, "hello {variable}");
                    *colorscheme_data += format!(
                        "\n  hl(0, \"{hl_group}\", {{ fg = {fg}, bg = {bg}, {style_options} }})",
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
        // TODO: handle error
        .expect("problem creating theme file");
}

// TODO: look into preserve order

// TODO: save palette keys don't allow if not in that list

// TODO: I hate how I'm updating this colorscheme string

// TODO: handle different backgrounds (dark, light)
fn main() {
    let args: ColorgenArgs = ColorgenArgs::parse();

    let content = std::fs::read_to_string(args.filename).unwrap();

    let template = content.parse::<Value>().unwrap();

    let name = template["information"]["name"].as_str().unwrap();

    let mut colorscheme_data = String::new();

    setup_directories(name);
    generate_init(name);
    generate_vim_colors_file(name);
    generate_palette(&template, name);
    generate_colorscheme(&template, &mut colorscheme_data);
    generate_theme(&colorscheme_data, name);
}

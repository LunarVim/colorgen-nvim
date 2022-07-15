use std::{env, fs};
use toml::Value;

// fn get_values(input: &Value) {
//     // TODO: remove this later
//     // fs::remove_dir("/home/chris/Repos/colorgen-nvim/lua").expect("Unable to remove dir");
//     match input.as_table() {
//         Some(table) => {
//             // TODO: skip for information and palette
//
//             // println!("tables");
//             // println!("{table:#?}");
//             let data = "Some data! \n\
//                                 test new line \n\
//                                 test another line \n";
//
//             // loop over files and add data
//             // for key in table.keys() {
//             //     .expect("Unable to write file");
//             // }
//
//             for (key, val) in table.iter() {
//                 // println!("{key}");
//                 get_values(val)
//                 // fs::write(
//                 //         format!(
//                 //             "{home_dir}/Repos/colorgen-nvim/{colorscheme_name}/lua/{colorscheme_name}/{filename}.lua",
//                 //             filename = key.as_str()
//                 //         ),
//                 //         data,
//                 //     )
//             }
//         }
//         None => {}
//     }
//
//     match input.as_str() {
//         Some(string) => {
//             // println!("string {string}")
//         }
//         None => {}
//     }
// }

fn setup_directories(name: &str) {
    fs::create_dir_all(format!(
        "{home_dir}/Repos/colorgen-nvim/{name}/lua/{name}",
        home_dir = env::var("HOME").unwrap(),
    ))
    .expect("Unable to write dir");
}

fn init(name: &str) {
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

fn create_colorscheme(template: &Value, name: &str) {}

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
      SignColumn = '- bg'
      MsgArea = 'fg bg'
      ModeMsg = 'fg dark'
      MsgSeparator = 'fg bg'
      SpellBad = 'light_red - u'
      SpellCap = 'yellow - u'
      SpellLocal = 'green - u'
    "#;

    let template = input.parse::<Value>().unwrap();

    let name = template["information"]["name"].as_str().unwrap();
    setup_directories(name);
    init(name);
    generate_palette(&template, name);
    create_colorscheme(&template, name);
}

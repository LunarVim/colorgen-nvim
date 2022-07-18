# colorgen-nvim

A colorscheme generator for Neovim

## Installing

For now we need to install via git using cargo

```sh
cargo install --git https://github.com/ChristianChiarulli/colorgen-nvim
```

## Usage

Create a toml file containing your highlight groups, there is an example in this repo called `user_template.toml`

```sh
colorgen-nvim user_template.toml
```

## Template

The template must contain a `information` section and a `palette` section

Example:

```toml
[information]
 name = "onedarker"
 background = "dark"
 author = 'Christian Chiarulli <chrisatmachine@gmail.com>'

[palette]
 fg = '#abb2bf'
 bg = '#1e222a'

 white = '#abb2bf'
 gray = '#545862'
 blue = '#519fdf'
 green = '#88b369'
 cyan = '#46a6b2'
 red = '#d05c65'
 orange = '#c18a56'
 yellow = '#d5b06b'
 purple = '#b668cd'
 magenta = '#D16D9E'
```

You can define color options in the palette section and use them later to set colors for different highlight groups

Example:

```toml
[highlights]
 Normal = 'fg bg'
 SignColumn = '- bg'
 MsgArea = 'fg bg'
 ModeMsg = 'fg bg'
 MsgSeparator = 'fg bg'
 SpellBad = 'red - u'
 SpellCap = 'yellow - u'
 SpellLocal = 'green - u'
 SpellRare = 'purple - u'
 NormalNC = 'fg bg'
 Pmenu = 'red bg'
 PmenuSel = '- blue'
 WildMenu = 'fg blue'
 CursorLineNr = 'light_gray - b'
 Comment = 'gray - i'
```

The format is `foreground background style`

The `-` is used skip a particular section and replace it with `NONE`

Style Options:

  - `o`: standout
  - `u`: underline
  - `c`: undercurl
  - `d`: underdouble
  - `t`: underdotted
  - `h`: underdashed
  - `s`: strikethrough
  - `i`: italic
  - `b`: bold
  - `r`: reverse
  - `n`: nocombine

## Inspiration and Credits

- [vim-felipec](https://github.com/felipec/vim-felipec)
- [ez.nvim](https://github.com/murtaza-u/ez.nvim)


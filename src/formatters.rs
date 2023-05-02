use crate::information::Background;
use std::fmt::{self, Display, Formatter};

pub struct InitLua<'a> {
    pub name: &'a str,
    pub background: Background,
}

impl<'a> Display for InitLua<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let background = &self.background;

        writeln!(
            f,
            r#"local M = {{}}
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

return M"#
        )
    }
}

pub struct VimColorsFile<'a> {
    pub name: &'a str,
}

impl<'a> Display for VimColorsFile<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = self.name;
        writeln!(f, r#"require("{name}").setup({{}})"#)
    }
}

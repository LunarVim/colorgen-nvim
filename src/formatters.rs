use crate::information::Background;
use std::fmt::{self, Display, Formatter};

pub struct InitLua<'a> {
    pub name: &'a str,
    pub background: Background,
}

impl<'a> Display for InitLua<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let background = self.background;

        writeln!(
            f,
            r#"local M = {{}}
local theme = require('{name}.theme')

M.setup = function()
{setup}

  theme.set_highlights()
end

return M"#,
            setup = InitSetup {
                name,
                background,
                indent: "  "
            }
        )
    }
}

pub struct InitSetup<'a> {
    pub name: &'a str,
    pub background: Background,
    pub indent: &'a str,
}

impl Display for InitSetup<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{indent}vim.cmd('hi clear')

{indent}vim.o.background = '{background}'
{indent}if vim.fn.exists('syntax_on') then
{indent}  vim.cmd('syntax reset')
{indent}end

{indent}vim.o.termguicolors = true
{indent}vim.g.colors_name = '{name}'"#,
            indent = self.indent,
            name = self.name,
            background = self.background
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

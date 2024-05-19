use self::parser::Color;
use bitflags::bitflags;
use std::fmt::Display;

pub mod parser;

#[derive(Debug)]
pub enum ColorSpec {
    Link(String),
    Color(ColorFormat),
}

impl Display for ColorSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorSpec::Link(link) => write!(f, "{{ link = '{link}' }}"),
            ColorSpec::Color(color) => write!(f, "{color}"),
        }
    }
}

#[derive(Debug)]
pub struct ColorFormat {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub style: Option<Style>,
    pub special: Option<Color>,
    pub blend: Option<u8>,
}

impl Display for ColorFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        if let Some(fg) = &self.fg {
            write!(f, " fg = {fg},")?;
        } else {
            write!(f, " fg = 'NONE',")?;
        }

        if let Some(bg) = &self.bg {
            write!(f, " bg = {bg},")?;
        } else {
            write!(f, " bg = 'NONE',")?;
        }

        if let Some(special) = &self.special {
            write!(f, " sp = {special},")?;
        }

        if let Some(blend) = &self.blend {
            write!(f, " blend = {blend},")?;
        }

        if let Some(style) = &self.style {
            write!(f, "{style}")?;
        }

        write!(f, " }}")?;
        Ok(())
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Style: u32 {
        const STANDOUT      = 1 <<  0;
        const UNDERLINE     = 1 <<  1;
        const UNDERCURL     = 1 <<  2;
        const UNDERDOUBLE   = 1 <<  3;
        const UNDERDOTTED   = 1 <<  4;
        const UNDERDASHED   = 1 <<  5;
        const STRIKETHROUGH = 1 <<  6;
        const ITALIC        = 1 <<  7;
        const BOLD          = 1 <<  8;
        const REVERSE       = 1 <<  9;
        const NOCOMBINE     = 1 << 10;
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for flag in self.iter() {
            match flag {
                Style::STANDOUT => write!(f, " standout=true,")?,
                Style::UNDERLINE => write!(f, " underline=true,")?,
                Style::UNDERCURL => write!(f, " undercurl=true,")?,
                Style::UNDERDOUBLE => write!(f, " underdouble=true,")?,
                Style::UNDERDOTTED => write!(f, " underdotted=true,")?,
                Style::UNDERDASHED => write!(f, " underdashed=true,")?,
                Style::STRIKETHROUGH => write!(f, " strikethrough=true,")?,
                Style::ITALIC => write!(f, " italic=true,")?,
                Style::BOLD => write!(f, " bold=true,")?,
                Style::REVERSE => write!(f, " reverse=true,")?,
                Style::NOCOMBINE => write!(f, " nocombine=true,")?,
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

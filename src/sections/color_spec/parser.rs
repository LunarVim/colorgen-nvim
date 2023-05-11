use crate::{
    palette::RgbColor,
    sections::color_spec::{ColorFormat, ColorSpec, Style},
};
use hex::FromHexError;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take, take_till},
    character::complete::digit1,
    combinator::{eof, map_res, rest},
    sequence::{pair, tuple},
    Finish, IResult, Parser,
};
use std::{fmt::Display, num::ParseIntError, str::FromStr};

mod serde;

impl FromStr for ColorSpec {
    type Err = ParseColorSpecError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, color_spec) = parse_color_spec(input)
            .finish()
            .map_err(|err| ParseColorSpecError(input.to_string(), err.to_string()))?;

        Ok(color_spec)
    }
}

#[derive(Debug, thiserror::Error)]
#[error(r#"failed to parse "{0}" with error "{1}""#)]
pub struct ParseColorSpecError(String, String);

fn parse_color_spec(input: &str) -> IResult<&str, ColorSpec> {
    let (input, color_spec) = alt((
        pair(tag("link:"), rest)
            .map(|(_, hl_group): (_, &str)| ColorSpec::Link(hl_group.to_string())),
        tuple((
            alt((
                eof.map(|_| None),
                tag("-").map(|_| None),
                parse_value.map(|color| Some(color)),
            )), // fg
            alt((
                eof.map(|_| None),
                tag(" -").map(|_| None),
                pair(tag(" "), parse_value).map(|(_, color)| Some(color)),
            )), // bg
            alt((
                eof.map(|_| None),
                tag(" -").map(|_| None),
                pair(tag(" "), parse_style).map(|(_, style)| Some(style)),
            )), // style
            alt((
                eof.map(|_| None),
                tag(" -").map(|_| None),
                pair(tag(" "), parse_value).map(|(_, color)| Some(color)),
            )), // sp
            alt((
                eof.map(|_| None),
                pair(tag(" -"), eof).map(|_| None),
                tuple((
                    tag(" "),
                    map_res(digit1, |s: &str| {
                        let blend = s.parse()?;
                        if (0..=100).contains(&blend) {
                            Ok(blend)
                        } else {
                            Err(ParseBlendError::RangeError(blend))
                        }
                    }),
                    eof,
                ))
                .map(|(_, blend, _)| Some(blend)),
            )),
        ))
        .map(|(fg, bg, style, special, blend)| {
            ColorSpec::Color(ColorFormat {
                fg,
                bg,
                style,
                special,
                blend,
            })
        }),
    ))(input)?;
    Ok((input, color_spec))
}

#[derive(Debug, thiserror::Error)]
enum ParseBlendError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("expected blend to be between 0 and 100, found {0}")]
    RangeError(u8),
}

#[derive(Debug, Clone)]
pub enum Color {
    Color(RgbColor),
    PaletteRef(String),
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Color(rgb_color) => write!(f, "'{rgb_color}'"),
            Color::PaletteRef(palette_value) => write!(f, "c.{palette_value}"),
        }
    }
}

fn parse_value(input: &str) -> IResult<&str, Color> {
    let (input, color) = alt((
        pair(
            tag("#"),
            map_res(take(6usize), |input| -> Result<Color, FromHexError> {
                let mut hex = [0u8; 3];
                hex::decode_to_slice(input, &mut hex)?;

                Ok(Color::Color(RgbColor(hex)))
            }),
        )
        .map(|(_, color)| color),
        take_till(char::is_whitespace).map(|str: &str| Color::PaletteRef(str.to_string())),
    ))(input)?;
    Ok((input, color))
}

fn parse_style(input: &str) -> IResult<&str, Style> {
    let (input, style_str) = is_a("oucdthsibrn")(input)?;
    Ok((
        input,
        style_str
            .chars()
            .map(|c| match c {
                'o' => Style::STANDOUT,
                'u' => Style::UNDERLINE,
                'c' => Style::UNDERCURL,
                'd' => Style::UNDERDOUBLE,
                't' => Style::UNDERDOTTED,
                'h' => Style::UNDERDASHED,
                's' => Style::STRIKETHROUGH,
                'i' => Style::ITALIC,
                'b' => Style::BOLD,
                'r' => Style::REVERSE,
                'n' => Style::NOCOMBINE,
                _ => unreachable!("The parser filters other characters out"),
            })
            .collect(),
    ))
}

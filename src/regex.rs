#![allow(dead_code, unused_variables, unused_macros, unused_mut)]

macro_rules! Sym {
    ($c:expr) => {
        RegexToken::Symbol($c)
    };
}

macro_rules! Star {
    ($c:expr) => {
        RegexToken::Star(Box::new($c))
    };
}

macro_rules! Plus {
    ($c:expr) => {
        RegexToken::Plus(Box::new($c))
    };
}

macro_rules! Concat {
    ($a:expr, $b:expr) => {
        RegexToken::Concat((Box::new($a), Box::new($b)))
    };
}

macro_rules! Union {
    ($a:expr, $b:expr) => {
        RegexToken::Union((Box::new($a), Box::new($b)))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(
            Regex::new(String::from("ab")),
            Concat!(Sym!('a'), Sym!('b'))
        )
    }

    #[test]
    fn test_plus() {
        assert_eq!(
            Regex::new(String::from("(a|b)+c")),
            Concat!(Plus!(Union!(Sym!('a'), Sym!('b'))), Sym!('c'))
        )
    }

    #[test]
    fn test_union() {
        assert_eq!(
            Regex::new(String::from("(a|b)")),
            Union!(Sym!('a'), Sym!('b'))
        )
    }

    #[test]
    fn test_none() {
        assert_eq!(Regex::new(String::from("")), RegexToken::None)
    }

    #[test]
    fn test_star() {
        assert_eq!(
            Regex::new(String::from("a*b")),
            Concat!(Star!(Sym!('a')), Sym!('b'))
        )
    }
}

type ReToken = Box<RegexToken>;

#[derive(Debug, PartialEq, Clone)]
pub enum RegexToken {
    Symbol(char),
    Concat((ReToken, ReToken)),
    Union((ReToken, ReToken)),
    Plus(ReToken),
    Star(ReToken),
    Dot,
    None,
}

#[derive(Debug, PartialEq)]
pub struct Regex {}

impl Regex {
    pub fn new(input: String) -> RegexToken {
        Regex::parse(input)
    }

    fn parse(input: String) -> RegexToken {
        if input.is_empty() {
            return RegexToken::None;
        }

        let mut chars = input.chars().peekable();
        let mut parsed_token = Self::parse_token(&mut chars);

        Self::parse_expression(&mut parsed_token, &mut chars)
    }

    fn parse_expression(
        left: &mut RegexToken,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> RegexToken {
        while let Some(&next) = chars.peek() {
            match next {
                '|' => {
                    chars.next(); // Consume '|'
                    let right = Self::parse_token(chars);
                    *left = RegexToken::Union((Box::new(left.clone()), Box::new(right)));
                }
                '*' => {
                    chars.next(); // Consume '*'
                    let right = Self::parse_token(chars);
                    *left = RegexToken::Concat((
                        Box::new(RegexToken::Star(Box::new(left.clone()))),
                        Box::new(right),
                    ));
                }
                '+' => {
                    chars.next(); // Consume '+'
                    let right = Self::parse_token(chars);
                    *left = RegexToken::Concat((
                        Box::new(RegexToken::Plus(Box::new(left.clone()))),
                        Box::new(right),
                    ));
                }
                _ => {
                    let right = Self::parse_token(chars);
                    if let RegexToken::None = right {
                        // do nothing
                    } else {
                        *left = RegexToken::Concat((Box::new(left.clone()), Box::new(right)));
                    }
                }
            }
        }
        left.clone()
    }

    fn parse_token(chars: &mut std::iter::Peekable<std::str::Chars>) -> RegexToken {
        match chars.next() {
            Some('(') => {
                let token = Self::parse(chars.collect());
                chars.next(); // Skip ')'
                token
            }
            Some('.') => RegexToken::Dot,
            Some(c) if c.is_ascii_alphanumeric() => Sym!(c),
            _ => RegexToken::None,
        }
    }
}

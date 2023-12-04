#![allow(dead_code, unused_variables, unused_macros, unused_mut)]

macro_rules! Sym {
    ($c:expr) => {
        Regex::Symbol($c)
    };
}

macro_rules! Star {
    ($c:expr) => {
        Regex::Star(Box::new($c))
    };
}

macro_rules! Plus {
    ($c:expr) => {
        Regex::Plus(Box::new($c))
    };
}

macro_rules! Concat {
    ($a:expr, $b:expr) => {
        Regex::Concat((Box::new($a), Box::new($b)))
    };
}

macro_rules! Union {
    ($a:expr, $b:expr) => {
        Regex::Union((Box::new($a), Box::new($b)))
    };
}

type RegexToken = Box<Regex>;

#[derive(Debug, PartialEq, Clone)]
pub enum Regex {
    Symbol(char),
    Concat((RegexToken, RegexToken)),
    Union((RegexToken, RegexToken)),
    Plus(RegexToken),
    Star(RegexToken),
    Dot,
    None,
}

impl Regex {
    pub fn new(input: String) -> Regex {
        Regex::parse(input)
    }

    fn parse(input: String) -> Regex {
        if input.is_empty() {
            return Regex::None;
        }

        let mut chars = input.chars().peekable();
        let mut parsed_token = Self::parse_token(&mut chars);

        Self::parse_expression(&mut parsed_token, &mut chars)
    }

    fn parse_expression(
        left: &mut Regex,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Regex {
        while let Some(&next) = chars.peek() {
            match next {
                '|' => {
                    chars.next(); // Consume '|'
                    let right = Self::parse_token(chars);
                    *left = Regex::Union((Box::new(left.clone()), Box::new(right)));
                }
                '*' => {
                    chars.next(); // Consume '*'
                    let right = Self::parse_token(chars);
                    *left = Regex::Concat((
                        Box::new(Regex::Star(Box::new(left.clone()))),
                        Box::new(right),
                    ));
                }
                '+' => {
                    chars.next(); // Consume '+'
                    let right = Self::parse_token(chars);
                    *left = Regex::Concat((
                        Box::new(Regex::Plus(Box::new(left.clone()))),
                        Box::new(right),
                    ));
                }
                _ => {
                    let right = Self::parse_token(chars);
                    if let Regex::None = right {
                        // do nothing
                    } else {
                        *left = Regex::Concat((Box::new(left.clone()), Box::new(right)));
                    }
                }
            }
        }
        left.clone()
    }

    fn parse_token(chars: &mut std::iter::Peekable<std::str::Chars>) -> Regex {
        match chars.next() {
            Some('(') => {
                let token = Self::parse(chars.collect());
                chars.next(); // Skip ')'
                token
            }
            Some('.') => Regex::Dot,
            Some(c) if c.is_ascii_alphanumeric() => Sym!(c),
            _ => Regex::None,
        }
    }
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
        assert_eq!(Regex::new(String::from("")), Regex::None)
    }

    #[test]
    fn test_star() {
        assert_eq!(
            Regex::new(String::from("a*b")),
            Concat!(Star!(Sym!('a')), Sym!('b'))
        )
    }
}

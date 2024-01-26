pub fn parse(source: &str) -> Option<Vec<Token>> {
    let mut remaining = source;
    let mut tokens = Vec::new();

    let mut look_ahead = 1;
    loop {
        let mut target = &remaining[..(look_ahead)];

        if ["\n", "\r"].contains(&target) {
            skip(&mut remaining, &mut look_ahead);
        }

        loop {
            if let Some(first) = target.chars().next() {
                if first.is_whitespace() {
                    target = &target[1..];
                    continue;
                }
            }
            break;
        }

        if let Some(tok) = Token::parse(target) {
            tokens.push(tok);
            skip(&mut remaining, &mut look_ahead);
        } else {
            look_ahead += 1;
        }
        if remaining.is_empty() {
            break;
        } else if look_ahead > remaining.len() {
            return None;
        }
    }

    Some(tokens)
}

fn skip(remaining: &mut &str, look_ahead: &mut usize) {
    *remaining = &remaining[*look_ahead..];
    *look_ahead = 1;
}

fn before<'a>(source: &'a str, end: &str) -> Option<&'a str> {
    inside(source, "", end)
}

fn between<'a>(source: &'a str, delimiters: &str) -> Option<&'a str> {
    inside(source, delimiters, delimiters)
}

fn inside<'a>(source: &'a str, before: &str, after: &str) -> Option<&'a str> {
    if source.len() < before.len() + after.len() {
        return None;
    }
    if source.starts_with(before) && source.ends_with(after) {
        Some(&source[(before.len())..(source.len() - after.len())])
    } else {
        None
    }
}

#[derive(Debug)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    Keyword(Keyword),
    Identifier(String),
    Literal(Literal),
}

impl Token {
    fn parse(source: &str) -> Option<Self> {
        None.or_else(|| Keyword::parse(source).map(Self::Keyword))
            .or_else(|| (source == "{").then_some(Self::OpenBrace))
            .or_else(|| (source == "}").then_some(Self::CloseBrace))
            .or_else(|| Literal::parse(source).map(Self::Literal))
            .or_else(|| Self::parse_identifier(source).map(Self::Identifier))
    }

    fn parse_identifier(source: &str) -> Option<String> {
        let before = before(source, " ")?;
        if before.chars().any(|c| !c.is_alphanumeric()) {
            return None;
        }
        Some(before.to_owned())
    }
}

#[derive(Debug)]
pub enum Keyword {
    System,

    On,
}

impl Keyword {
    fn parse(source: &str) -> Option<Self> {
        let before = before(source, " ")?;
        Some(match before {
            "system" => Self::System,
            "on" => Self::On,
            _ => return None,
        })
    }
}

#[derive(Debug)]
pub enum Literal {
    String(String),
}

impl Literal {
    fn parse(source: &str) -> Option<Self> {
        Some(Self::String(between(source, "\"")?.to_owned()))
    }
}

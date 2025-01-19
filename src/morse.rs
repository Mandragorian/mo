#[derive(Debug)]
pub enum MorseSymbol {
    Dit,
    Dah,
}

pub struct EndodedChar(Vec<MorseSymbol>);

impl std::fmt::Display for EndodedChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for symbol in self.0.iter() {
            let c = match symbol {
                MorseSymbol::Dit => "•",
                MorseSymbol::Dah => "—",
            };
            write!(f, "{} ", c)?; 
        }
        for _ in 0..(5-self.0.len()) {
            write!(f, "  ")?; 
        }
        Ok(())
    }
}

pub fn decode_symbols(symbols: &Vec<MorseSymbol>) -> Option<char> {
    let res = match symbols.as_slice() {
        [MorseSymbol::Dit, MorseSymbol::Dah] => Some('a'),
        [MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dit] => Some('b'),
        [MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dah, MorseSymbol::Dit] => Some('c'),
        [MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dit] => Some('d'),
        [MorseSymbol::Dit] => Some('e'),
        [MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dah, MorseSymbol::Dit] => Some('f'),
        [MorseSymbol::Dah, MorseSymbol::Dah, MorseSymbol::Dit] => Some('g'),
        [MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dit] => Some('h'),
        [MorseSymbol::Dit, MorseSymbol::Dit] => Some('i'),
        [MorseSymbol::Dit, MorseSymbol::Dah, MorseSymbol::Dah, MorseSymbol::Dah] => Some('j'),
        [MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dah] => Some('k'),
        [MorseSymbol::Dit, MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dit] => Some('l'),
        [MorseSymbol::Dah, MorseSymbol::Dah] => Some('m'),
        [MorseSymbol::Dah, MorseSymbol::Dit] => Some('n'),
        [MorseSymbol::Dah, MorseSymbol::Dah, MorseSymbol::Dah] => Some('o'),
        [MorseSymbol::Dit, MorseSymbol::Dah, MorseSymbol::Dah, MorseSymbol::Dit] => Some('p'),
        [MorseSymbol::Dah, MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dah] => Some('q'),
        [MorseSymbol::Dit, MorseSymbol::Dah, MorseSymbol::Dit] => Some('r'),
        [MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dit] => Some('s'),
        [MorseSymbol::Dah] => Some('t'),
        [MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dah] => Some('u'),
        [MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dah] => Some('v'),
        [MorseSymbol::Dit, MorseSymbol::Dah, MorseSymbol::Dah] => Some('w'),
        [MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dah] => Some('x'),
        [MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dah, MorseSymbol::Dah] => Some('y'),
        [MorseSymbol::Dah, MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dit] => Some('z'),
        _ => None,
    };
    res
}

pub fn encode_character(character: char) -> Option<EndodedChar> {
    Some(EndodedChar(match character {
        'a' => vec![MorseSymbol::Dit, MorseSymbol::Dah],
        'b' => vec![MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dit],
        'c' => vec![MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dah, MorseSymbol::Dit],
        'd' => vec![MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dit],
        'e' => vec![MorseSymbol::Dit],
        'f' => vec![MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dah, MorseSymbol::Dit],
        'g' => vec![MorseSymbol::Dah, MorseSymbol::Dah, MorseSymbol::Dit],
        'h' => vec![MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dit],
        'i' => vec![MorseSymbol::Dit, MorseSymbol::Dit],
        'j' => vec![MorseSymbol::Dit, MorseSymbol::Dah, MorseSymbol::Dah, MorseSymbol::Dah],
        'k' => vec![MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dah],
        'l' => vec![MorseSymbol::Dit, MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dit],
        'm' => vec![MorseSymbol::Dah, MorseSymbol::Dah],
        'n' => vec![MorseSymbol::Dah, MorseSymbol::Dit],
        'o' => vec![MorseSymbol::Dah, MorseSymbol::Dah, MorseSymbol::Dah],
        'p' => vec![MorseSymbol::Dit, MorseSymbol::Dah, MorseSymbol::Dah, MorseSymbol::Dit],
        'q' => vec![MorseSymbol::Dah, MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dah],
        'r' => vec![MorseSymbol::Dit, MorseSymbol::Dah, MorseSymbol::Dit],
        's' => vec![MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dit],
        't' => vec![MorseSymbol::Dah],
        'u' => vec![MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dah],
        'v' => vec![MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dah],
        'w' => vec![MorseSymbol::Dit, MorseSymbol::Dah, MorseSymbol::Dah],
        'x' => vec![MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dit, MorseSymbol::Dah],
        'y' => vec![MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dah, MorseSymbol::Dah],
        'z' => vec![MorseSymbol::Dah, MorseSymbol::Dah, MorseSymbol::Dit, MorseSymbol::Dit],
        _ => return None,
    }))
}

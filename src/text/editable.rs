use sdl2::keyboard::Keycode;

#[derive(Debug, Clone)]
pub struct Text {
    text: String,
    cursor: usize,
}

impl Text {
    pub const fn new() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
        }
    }

    pub fn insert(&mut self, c: impl Insertable, shift_pressed: bool) {
        if let Some(c) = c.into_char(shift_pressed) {
            self.text.insert(self.cursor, c);
            self.cursor += 1;
        }
    }

    pub fn pop(&mut self) {
        if self.cursor > 0 {
            self.text.remove(self.cursor - 1);
            self.cursor -= 1;
        }
    }
}

impl From<String> for Text {
    fn from(text: String) -> Self {
        Self {
            cursor: text.len(),
            text,
        }
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

pub trait Insertable {
    fn into_char(self, shift_pressed: bool) -> Option<char>;
}

impl Insertable for Keycode {
    fn into_char(self, shift_pressed: bool) -> Option<char> {
        macro_rules! map_key {
            ($($enum_name: ident => $char: expr),*) => {
                use sdl2::keyboard::Keycode::{$($enum_name),*};
                let ret = match self {
                    $($enum_name => Some($char),)*
                    _ => None
                };
                if shift_pressed {
                    match ret {
                        Some(c) => Some(c.to_ascii_uppercase()),
                        None => None
                    }
                } else {ret}
            };
        }
        map_key! {
            A => 'a',
            B => 'b',
            C => 'c',
            D => 'd',
            E => 'e',
            F => 'f',
            G => 'g',
            H => 'h',
            I => 'i',
            J => 'j',
            K => 'k',
            L => 'l',
            M => 'm',
            N => 'n',
            O => 'o',
            P => 'p',
            Q => 'q',
            R => 'r',
            S => 's',
            T => 't',
            U => 'u',
            V => 'v',
            W => 'w',
            X => 'x',
            Y => 'y',
            Z => 'z',
            Space => ' ',
            Return => '\n',
            Tab => '\t'
        }
    }
}

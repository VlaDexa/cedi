use std::rc::Rc;

use crate::text::editable::Text;

#[derive(Debug)]
pub struct Side {
    text: Text,
    left: Option<Rc<Side>>,
    right: Option<Rc<Side>>,
    up: Option<Rc<Side>>,
    down: Option<Rc<Side>>,
}

impl Side {
    pub const fn new(text: Text) -> Self {
        Self {
            text,
            left: None,
            right: None,
            up: None,
            down: None,
        }
    }
}

impl AsRef<Text> for Side {
    fn as_ref(&self) -> &Text {
        &self.text
    }
}

impl AsMut<Text> for Side {
    fn as_mut(&mut self) -> &mut Text {
        &mut self.text
    }
}

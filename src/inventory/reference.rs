use super::item::Item;

pub struct Reference;

impl Item for Reference {
    fn icon(&self) -> char {
        '&'
    }
}

impl Reference {
    pub fn new() -> Reference {
        Reference
    }
}
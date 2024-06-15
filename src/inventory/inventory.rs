use super::item::{Item, Items};

pub struct Inventory {
    items: Vec<Items>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            items: Vec::new(),
        }
    }

    pub fn items(&self) -> &Vec<Items> {
        &self.items
    }

    pub fn add_item(&mut self, item: Items) {
        self.items.push(item);
    }
}
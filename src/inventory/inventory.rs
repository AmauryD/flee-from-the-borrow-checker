use super::item::Item;

pub struct Inventory {
    items: Vec<Box<dyn Item>>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            items: Vec::new(),
        }
    }

    pub fn items(&self) -> &Vec<Box<dyn Item>> {
        &self.items
    }

    pub fn add_item(&mut self, item: Box<dyn Item>) {
        self.items.push(item);
    }
}
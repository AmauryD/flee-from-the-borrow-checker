use super::reference::Reference;

pub trait Item {
    fn icon(&self) -> char;
}

pub enum Items {
    Reference(Reference)
}

impl Item for Items {
    fn icon(&self) -> char {
        match self {
            Items::Reference(r) => r.icon()
        }
    }
}
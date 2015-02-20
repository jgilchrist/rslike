use std::slice::Iter;

pub struct Menu<T> {
    menu_items: Vec<T>,
    selected: usize,
}

impl<T> Menu<T> {
    pub fn new(menu_items: Vec<T>) -> Menu<T> {
        Menu { menu_items: menu_items, selected: 0 }
    }

    pub fn next(&mut self) {
        if self.selected < self.menu_items.len() - 1 {
            self.selected += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn selected(&self) -> &T {
        &self.menu_items[self.selected]
    }

    pub fn is_selected(&self, index: usize) -> bool {
        self.selected == index        
    }

    pub fn items(&self) -> Iter<T> {
        self.menu_items.iter()
    }
}

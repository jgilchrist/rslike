use std::slice::Iter;

pub struct Menu<T> {
    menu_items: Vec<MenuOption<T>>,
    selected: usize,
}

pub struct MenuOption<T>(pub &'static str, pub T);

impl<T> MenuOption<T> {
    pub fn text(&self) -> &str {
        &self.0
    }

    pub fn option(&self) -> &T {
        &self.1
    }

}

impl<T> Menu<T> {
    pub fn new(menu_items: Vec<MenuOption<T>>) -> Menu<T> {
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

    pub fn selected(&self) -> &MenuOption<T> {
        &self.menu_items[self.selected]
    }

    pub fn is_selected(&self, index: usize) -> bool {
        self.selected == index        
    }

    pub fn items(&self) -> Iter<MenuOption<T>> {
        self.menu_items.iter()
    }
}

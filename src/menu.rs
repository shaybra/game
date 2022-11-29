pub struct GameMenu {
    pub is_visible: bool,
}

impl GameMenu {
    pub fn new(is_visible: bool) -> GameMenu {
        GameMenu { is_visible }
    }

    pub fn show_menu(&mut self, size: (u16, u16)) {
        self.is_visible = true;
        // get the middle of the screen

        // calculate the percentage of the screen which the menu will take

        // calculate where to start the population and end it

        // populate the screen with the menu
    }
}

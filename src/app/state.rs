#[derive(Clone, PartialEq)]
pub enum GameState {
    MainMenu,
    NewGameEnterWidth(NewGameState),
    NewGameEnterHeight(NewGameState),
    NewGameEnterName(NewGameState),
    LoadGame,
    Playing,
}

#[derive(Clone, PartialEq)]
pub struct NewGameState {
    pub width_input: String,
    pub height_input: String,
    pub name_input: String,
}

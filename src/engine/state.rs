pub enum GameState {
    MainMenu, Dealing, Round(usize, usize, bool, bool, bool), Resolving, Collecting
}
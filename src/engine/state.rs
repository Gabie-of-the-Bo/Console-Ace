pub enum GameState {
    MainMenu, Dealing, Round(usize), Resolving, Collecting
}
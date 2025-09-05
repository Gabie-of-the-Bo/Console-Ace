pub enum GameState {
    MainMenu, Dealing, Round(usize, usize), Resolving, Collecting
}
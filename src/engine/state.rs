pub enum GameState {
    MainMenu(bool), Dealing, Round(usize, usize, bool, bool, bool), Resolving, Collecting, End(bool)
}
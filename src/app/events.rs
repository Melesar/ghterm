pub enum AppEvent {
    RepoChosen (u32),
    Error(String),
    ScreenRepaint,
    Input(termion::event::Key),
    TaskCompleted,
}

use druid::Data;

#[derive(Clone, PartialEq, Data)]
pub enum GameEndState {
    Loss,
    Win
}

#[derive(Clone, Data)]
pub enum GameState {
    NotStarted,
    Started,
    Paused,
    EndState(GameEndState)
}
use druid::Data;

#[derive(Clone, PartialEq, Data)]
pub enum GameEndState {
    Loss,
    Win,
}

#[derive(Clone, Data, PartialEq)]
pub enum GameState {
    NotStarted,
    Running,
    Paused,
    EndState(GameEndState),
}

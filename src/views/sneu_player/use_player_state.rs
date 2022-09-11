use yew::prelude::*;

#[derive(PartialEq, Eq, Clone, Properties)]
pub struct PlayerState {
    pub is_playing: bool,
    pub is_muted: bool,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            is_playing: true,
            is_muted: true,
        }
    }
}

pub enum PlayerAction {
    TogglePlaying,
    ToggleMuted,
}

impl Reducible for PlayerState {
    type Action = PlayerAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            PlayerAction::TogglePlaying => Self {
                is_playing: !self.is_playing,
                ..*self
            },
            PlayerAction::ToggleMuted => Self {
                is_muted: !self.is_muted,
                ..*self
            },
        }
        .into()
    }
}

pub fn use_player_state() -> UseReducerHandle<PlayerState> {
    use_reducer_eq(PlayerState::default)
}

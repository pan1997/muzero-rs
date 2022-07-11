



pub trait SearchProblem {
    type HiddenState: HiddenState<Self::Player, Self::Action>;
    type Action;
    type Observation: Observation<Self::Player, Self::Action>;
    type Player: Copy;

    fn get_observation<'a>(
        &self,
        state: &'a Self::HiddenState,
        player: Self::Player,
    ) -> &'a Self::Observation;

    fn get_all_players(&self) -> Vec<Self::Player>;

    // Translates an action from one players point of view to another.
    // to support Partially observable actions
    fn get_visible_action(
        &self,
        state: &Self::HiddenState,
        action: &Self::Action,
        player: &Self::Player
    ) -> Self::Action;
}


pub trait Observation<Player: Copy, Action> {
    // last reward for current
    fn reward(&self) -> f32;

    fn legal_actions(&self) -> Vec<Action>;
}

pub trait HiddenState<Player, Action> {
    fn apply(&self, action: &Action) -> Self;
    // current actor is unambiguous
    fn current_actor(&self) -> Player;

    fn is_terminal(&self) -> bool;
}
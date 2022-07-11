use std::fmt::{Display, Formatter};
use std::ptr::write;
use rand::Rng;
use rand::seq::SliceRandom;
use crate::lib::search_problem::{Observation, SearchProblem};
use crate::lib::search_problem::HiddenState;
use crate::lib::Simulator;

struct TwoZeroFourEight {

}

#[derive(Clone)]
struct Board {
    cells: [[u32; 4]; 4],
    // total of all the newly created tiles
    new_tile_sum: u32,
    // if this state is terminal
    terminal: bool,
    // if random tile has been dropped
    dropped: bool,
}

#[derive(Copy, Clone, Debug)]
enum Player {
    Environment,
    Agent
}

#[derive(Copy, Clone, Debug)]
enum Action {
    Left,
    Right,
    Up,
    Down,

    Place(usize, usize) // Move of environment player
}

const ALL_ACTIONS: [Action; 4] = [Action::Left, Action::Right, Action::Up, Action::Down];


impl SearchProblem for TwoZeroFourEight {
    type HiddenState = Board;
    type Action = Action;
    type Observation = Board;
    type Player = Player;

    fn get_observation<'a>(
        &self,
        state: &'a Board,
        _: Player, // All players have full visibility
    ) -> &'a Self::Observation {
        state
    }

    fn get_all_players(&self) -> Vec<Self::Player> {
        vec![Player::Environment, Player::Agent]
    }

    fn get_visible_action(
        &self,
        _: &Self::HiddenState,
        action: &Self::Action,
        _: &Self::Player
    ) -> Self::Action {
        // No partially observable actions
        action.clone()
    }
}


impl Observation<Player, Action> for Board {
    fn reward(&self) -> f32 {
        self.new_tile_sum as f32
    }


    fn legal_actions(&self) -> Vec<Action> {
        // All actions are legal at all states (except when its terminal)
        if self.is_terminal() {
            vec![]
        } else if self.dropped {
            ALL_ACTIONS.to_vec()
        } else {
            let mut result = vec![];
            for row in 0..4 {
                for col in 0..4 {
                    if self.cells[row][col] == 0 {
                        result.push(Action::Place(row, col))
                    }
                }
            }
            result
        }
    }
}

impl HiddenState<Player, Action> for Board {
    fn apply(&self, action: &Action) -> Self {
        match action {
            Action::Left => self.shift_left(),
            Action::Right => self.shift_right(),
            Action::Up => self.shift_up(),
            Action::Down => self.shift_down(),
            Action::Place(row, col) => {
                let mut result = self.clone();
                result.cells[*row][*col] = 2;
                result.dropped = true;
                result
            }
        }
    }

    fn current_actor(&self) -> Player {
        if self.dropped {
            Player::Agent
        } else {
            Player::Environment
        }
    }

    fn is_terminal(&self) -> bool {
        // A state is terminal if it is same as the parent state
        self.terminal
    }
}


impl Board {
    fn new() -> Board {
        Board{
            cells: [[0; 4]; 4],
            new_tile_sum: 0,
            terminal: false,
            dropped: true,
        }
    }

    fn shift_left(&self) -> Board {
        let mut result = Board::new();
        let mut changed = false;
        let mut new_tile_sum = 0;

        let mut empty_cell_count: u32 = 0;

        for row in 0..4 {
            let mut pos = 0;
            let mut skip_flag = true;
            for col in 0..4 {
                if self.cells[row][col] != 0 {
                    if !skip_flag && pos >= 1 && self.cells[row][col] == result.cells[row][pos-1] {
                        result.cells[row][pos-1] *= 2;
                        new_tile_sum += result.cells[row][pos-1];
                        changed = true;
                        skip_flag = true;
                        // one less cell in result
                        empty_cell_count += 1;
                    } else {
                        result.cells[row][pos] = self.cells[row][col];
                        if pos != col {
                            changed = true;
                        }
                        pos += 1;
                        skip_flag = false;
                    }
                } else {
                    empty_cell_count += 1;
                }
            }
        }
        result.new_tile_sum = new_tile_sum;
        // the resulting board is terminal if it was not changed
        result.terminal = !changed;
        result.dropped = false;
        result
    }

    fn transpose(&mut self) {
        for row in 0..4 {
            for col in (row+1)..4 {
                let t = self.cells[row][col];
                self.cells[row][col] = self.cells[col][row];
                self.cells[col][row] = t;
            }
        }
    }

    fn flip_vertically(&mut self) {
        for row in 0..4 {
            for col in 0..2 {
                let t = self.cells[row][col];
                self.cells[row][col] = self.cells[row][3-col];
                self.cells[row][3-col] = t;
            }
        }
    }

    fn shift_right(&self) -> Board {
        let mut b = self.clone();
        b.flip_vertically();
        let mut result = b.shift_left();
        result.flip_vertically();
        result
    }

    fn shift_up(&self) -> Board {
        let mut b = self.clone();
        b.transpose();
        let mut result = b.shift_left();
        result.transpose();
        result
    }

    fn shift_down(&self) -> Board {
        let mut b = self.clone();
        b.transpose();
        let mut result = b.shift_right();
        result.transpose();
        result
    }
}


impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..4 {
            for col in 0..4 {
                write!(f, "|{:5}", self.cells[row][col])?;
            }
            write!(f, "|\n")?;
        }
        if self.terminal {
            write!(f, "<>, {}\n", self.new_tile_sum)?;
        } else {
            write!(f, "><, {}\n", self.new_tile_sum)?;
        }
        Ok(())
    }
}

struct TwoZeroFourEightSimulator;

impl Simulator<TwoZeroFourEight> for TwoZeroFourEightSimulator {
    fn simulate(&self, problem: &TwoZeroFourEight, state: &Board, horizon: u32, discount: f32) -> Vec<(Player, f32)> {

        assert!(state.dropped, "Environment player cannot be the agent to move");

        let mut total_score: f32 = 0.0;

        let mut discount_factor = 1.0;
        let mut current_state = state.clone();

        for _ in 0..horizon {
            //println!("{}: \n{}", index, current_state);

            let obs = problem.get_observation(&current_state,Player::Agent);
            total_score += obs.reward() * discount_factor;

            if obs.is_terminal() {
                break;
            }

            let mut actions = obs.legal_actions();
            actions.shuffle(&mut rand::thread_rng());
            let mut all_terminal = true;
            'inner: for action in actions {
                //println!("tried {:?}", action);
                let current_state_temp = current_state.apply(&action);
                if !current_state_temp.is_terminal() {
                    current_state = current_state_temp;
                    all_terminal = false;
                    assert!(!current_state.dropped, "expected environment");
                    let environment_player_actions = current_state.legal_actions();
                    let environment_player_action = environment_player_actions.choose(&mut rand::thread_rng()).unwrap();
                    current_state = current_state.apply(environment_player_action);
                    break 'inner;
                }
            }
            if all_terminal {
                break;
            }
            discount_factor *= discount;
        }
        vec![(Player::Environment, 0.0), (Player::Agent, total_score)]
    }
}


#[cfg(test)]
mod test {
    use crate::lib::Simulator;
    use crate::lib::search_problem::HiddenState;
    use crate::lib::tzf8::{Action, Board, TwoZeroFourEight, TwoZeroFourEightSimulator};

    fn board1() -> Board {
        let mut  b = Board::new();
        b.cells[0][0] = 2;
        b.cells[0][1] = 2;
        b.cells[1][2] = 4;
        b.cells[0][3] = 4;
        b.cells[1][0] = 2;
        b.cells[2][3] = 4;
        b
    }

    #[test]
    fn t1() {
        let b = board1();
        println!("{}", b);
        println!("{}", b.apply(&Action::Left));
        println!("{}", b.apply(&Action::Right));
        println!("{}", b.apply(&Action::Up));
        println!("{}", b.apply(&Action::Down));
    }

    #[test]
    fn t2() {
        let b = board1();
        let sim = TwoZeroFourEightSimulator{};
        let p = TwoZeroFourEight{};

        let result = sim.simulate(&p, &b, 1000, 1.0);

        println!("{:?}", result);
    }
}
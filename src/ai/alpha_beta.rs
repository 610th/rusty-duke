//! Implments alpha beta agent for the Rusty Duke game.

use crate::logic::{
    self, do_unsafe_action_copy, get_actions, get_spawn_squares, get_tile_actions, Action,
    ActionResult, ActionType, AvailableAction, AvailableEffect, Coordinate, Effect, GameState,
    IntoEnumIterator, TileColor, TileType, Winner, HEIGHT, TILE_ACTIONS, TILE_EFFECTS, WIDTH,
};
use log::debug;
use std::cmp::Ordering;
use std::collections::HashMap;
pub use std::time::Duration;
use std::time::Instant;

#[derive(PartialEq, Debug)]
pub struct Agent {
    pub color: TileColor,
    pub depth: Option<u8>,          /* Search depth */
    pub duration: Option<Duration>, /* Max search duration */
}

impl Agent {
    /// Create new agent. Depth and/or duration has to be set.
    pub fn new(color: TileColor, depth: Option<u8>, duration: Option<Duration>) -> Agent {
        assert!(
            depth.is_some() || duration.is_some(),
            "Depth and/or duration has to be set."
        );

        Agent {
            color: color,
            depth: depth,
            duration: duration,
        }
    }
}

/// Compare heuristics of two actions. Greater is better.
fn action_cmp(a: &(&GameState, Action), b: &(&GameState, Action)) -> std::cmp::Ordering {
    // To be extended with heuristics

    let state = a.0;
    let a = &a.1;
    let b = &b.1;

    match a {
        Action::NewFromBag => {
            match b {
                Action::NewFromBag => {
                    return Ordering::Equal;
                }
                Action::PlaceNew(_) => {
                    // Should never happen
                    return Ordering::Less;
                }
                Action::Move(r_data)
                | Action::Jump(r_data)
                | Action::JumpSlide(r_data)
                | Action::Slide(r_data)
                | Action::Strike(r_data) => {
                    return if r_data.result == ActionResult::Capture {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    };
                }
                Action::Command(r_data) => {
                    return if r_data.result == ActionResult::Capture {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    };
                }
            }
        }
        Action::PlaceNew(_) => {
            match b {
                Action::PlaceNew(_) => {
                    // Should never happen
                    return Ordering::Equal;
                }
                _ => {
                    return Ordering::Greater;
                }
            }
        }
        _ => {
            // Hack to handle both Action and Command data.

            let l_result;
            let r_result;
            let l_target_pos;
            let r_target_pos;

            match a {
                Action::Move(l_data)
                | Action::Jump(l_data)
                | Action::Slide(l_data)
                | Action::JumpSlide(l_data)
                | Action::Strike(l_data) => {
                    l_result = l_data.result;
                    l_target_pos = l_data.target_pos;
                }
                Action::Command(l_data) => {
                    l_result = l_data.result;
                    l_target_pos = l_data.target_pos;
                }
                _ => {
                    panic!("Illegal action.");
                }
            }

            match b {
                Action::Move(r_data)
                | Action::Jump(r_data)
                | Action::Slide(r_data)
                | Action::JumpSlide(r_data)
                | Action::Strike(r_data) => {
                    r_result = r_data.result;
                    r_target_pos = r_data.target_pos;
                }
                Action::Command(r_data) => {
                    r_result = r_data.result;
                    r_target_pos = r_data.target_pos;
                }
                Action::PlaceNew(_) => {
                    return Ordering::Less;
                }
                _ => {
                    return Ordering::Equal;
                }
            }

            if l_result == ActionResult::Capture {
                let l_capture_tile = state.square(l_target_pos).tile.as_ref().unwrap();
                match b {
                    Action::NewFromBag => {
                        return Ordering::Greater;
                    }
                    Action::PlaceNew(_) => {
                        return Ordering::Less;
                    }
                    Action::Move(_)
                    | Action::Jump(_)
                    | Action::JumpSlide(_)
                    | Action::Slide(_)
                    | Action::Strike(_)
                    | Action::Command(_) => {
                        if r_result == ActionResult::Capture {
                            let r_capture_tile = state.square(r_target_pos).tile.as_ref().unwrap();
                            if TILE_UTILITY.get(&l_capture_tile.kind)
                                < TILE_UTILITY.get(&r_capture_tile.kind)
                            {
                                return Ordering::Less;
                            } else if TILE_UTILITY.get(&l_capture_tile.kind)
                                > TILE_UTILITY.get(&r_capture_tile.kind)
                            {
                                return Ordering::Greater;
                            }
                            return Ordering::Equal;
                        }
                        return Ordering::Less;
                    }
                }
            }

            match b {
                Action::PlaceNew(_) => {
                    return Ordering::Less;
                }
                _ => {
                    return Ordering::Equal;
                }
            }
        }
    }
}

/// Naive effort to calculate utility of tile type. Tune for better AI.
fn tile_utility(kind: TileType) -> i32 {
    let mut utility: i32 = 0;

    // Test special high utility for duke.
    if kind == TileType::Duke {
        return 1000;
    }

    let utility_from_actions = |actions: &Vec<AvailableAction>| -> i32 {
        let mut u = 0;
        for a in actions {
            match a.kind {
                ActionType::Move => {
                    u = u + 1;
                }
                ActionType::Jump => {
                    u = u + 3;
                }
                ActionType::JumpSlide => {
                    u = u + 4;
                }
                ActionType::Slide => {
                    u = u + 2;
                }
                ActionType::Command => {
                    u = u + 2;
                }
                ActionType::Strike => {
                    u = u + 3;
                }
                _ => {}
            }
        }
        u
    };

    let utility_from_effects = |effects: &Vec<AvailableEffect>| -> i32 {
        let mut u = 0;
        for e in effects {
            match e.kind {
                Effect::Dread => {
                    u = u + 1;
                }
                Effect::Defence => {
                    u = u + 3;
                }
            }
        }
        u
    };

    utility = utility + utility_from_actions(&TILE_ACTIONS[&kind].front);
    utility = utility + utility_from_actions(&TILE_ACTIONS[&kind].back);

    // Most tiles does not have effects.
    if TILE_EFFECTS.get(&kind).is_some() {
        utility = utility + utility_from_effects(&TILE_EFFECTS[&kind].front);
        utility = utility + utility_from_effects(&TILE_EFFECTS[&kind].back);
    }

    utility
}

lazy_static! {
    // FIXME: Find suitable hash algorithm.
    static ref TILE_UTILITY: HashMap<TileType, i32> = {
        let mut m = HashMap::new();
        for kind in TileType::iter() {
            m.insert(
                kind,
                tile_utility(kind)
            );
        }
        m
    };
}

/// Evaluation function with super naive heuristics. Returns utility of game
/// state for agent. High utility is better.
fn utility(agent: &Agent, state: &logic::GameState) -> i32 {
    // First, check if end game

    if state.game_over.is_some() {
        let winner = state.game_over.as_ref();

        match winner {
            //Some(Winner::Draw) => return 20, // FIXME: There is no draw?
            Some(Winner::Color(c)) => {
                if *c == agent.color {
                    return 1000000;
                } else {
                    return -1000000;
                }
            }
            _ => {
                panic!("Can't be None.")
            }
        };
    }

    // Calculate utility of game state
    let mut utility: i32 = 0;
    const CHECK_MATE_UTIL: i32 = 100000;

    let check_mate = |result: ActionResult, target_pos: Coordinate| {
        if result == ActionResult::Capture {
            let tile = state.square(target_pos).tile.as_ref().unwrap();
            if tile.kind == TileType::Duke {
                if tile.color == agent.color {
                    if state.ply == agent.color {
                        // Agent is checked.
                        return -1000;
                    } else {
                        // Agent is check mate.
                        return -CHECK_MATE_UTIL;
                    }
                } else {
                    if state.ply == agent.color {
                        // Opponent is check mate.
                        return CHECK_MATE_UTIL;
                    } else {
                        // Opponent is checked.
                        return 1000;
                    }
                }
            }
        }
        return 0;
    };

    // Get value from tiles on board.
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let cord = Coordinate::new(x as u8, y as u8);
            let tile = state.square(cord).tile;
            if tile.is_some() {
                let tile = tile.as_ref().unwrap();

                // Check if duke is check [mate].
                let actions = get_tile_actions(state, cord);
                for a in actions {
                    match a {
                        Action::Move(ad)
                        | Action::Jump(ad)
                        | Action::JumpSlide(ad)
                        | Action::Slide(ad)
                        | Action::Strike(ad) => {
                            utility = utility + check_mate(ad.result, ad.target_pos);
                        }
                        Action::Command(cd) => {
                            utility = utility + check_mate(cd.result, cd.target_pos);
                        }
                        _ => {}
                    }

                    // Stop if check mate
                    if utility.abs() >= CHECK_MATE_UTIL {
                        return utility;
                    }
                }

                // Add value from tile
                if tile.color == agent.color {
                    utility = utility + TILE_UTILITY.get(&tile.kind).unwrap();
                } else {
                    utility = utility - TILE_UTILITY.get(&tile.kind).unwrap();
                }
            }
        }
    }

    // Digg through the graves as well.
    // Dead friends are bad friends. Dead enemies are good enemies.
    /*for t in state.graveyard.iter() {
        if t.color == agent.color {
            utility = utility - TILE_UTILITY.get(&t.kind).unwrap();
        } else {
            utility = utility + TILE_UTILITY.get(&t.kind).unwrap();
        }
    }*/

    // I guess that spawn square is worth 5.
    utility = utility + get_spawn_squares(state).len() as i32 * 5;

    return utility;
}

struct Timer {
    duration: Duration,
    start: Instant,
}

fn try_branch(
    agent: &Agent,
    state: &GameState,
    alpha: i32,
    beta: i32,
    depth: u8,
    timer: Option<&Timer>,
    max: bool,
    action: &Action,
) -> (Option<Action>, i32) {
    match action {
        Action::NewFromBag => {
            // This is hard because the action involves chance.
            // Special case, because this action is 2 stage.

            let mut u = 0;
            for t in state.bag().iter() {
                u = u + TILE_UTILITY.get(&t.kind).unwrap();
            }
            u = u / state.bag().len() as i32;
            u = u + utility(agent, state);
            return (None, u);

            // Do manual Action::NewFromBag for every tile in bag. And take
            // average of utility. Only do shallow search for every tile.
            // If bag tiles are included in utility calc, this has to be updated.
            /*let mut copy_state = state.clone();
            let tiles_in_bag = copy_state.bag().len() as i32;

            if tiles_in_bag == 0 {
                panic!("NewFromBag but no tiles in bag.");
            }

            if ! copy_state.drawn().is_empty() {
                panic!("NewFromBag but drawn tiles.");
            }

            let mut u = 0;
            while ! copy_state.bag().is_empty() {
                let t = copy_state.mut_bag().pop().unwrap();
                copy_state.mut_drawn().push(t);
                let actions = get_actions(&copy_state);
                for a in actions {
                    match a {
                        Action::PlaceNew(_) => {}
                        _ => {panic!("Not PlaceNew!");}
                    }
                    let new_state = do_unsafe_action_copy(&copy_state, &a);
                    // Just do a shallow search here
                    //let (_, u) = alpha_beta(agent, &new_state, alpha, beta, 1, timer, max);
                    u = u + utility(agent, &new_state);
                }
                copy_state.mut_drawn().clear();
            }

            // Return average
            return (None, u / tiles_in_bag);*/
        }
        _ => {}
    }

    let new_state = do_unsafe_action_copy(state, &action);
    alpha_beta(agent, &new_state, alpha, beta, depth, timer, max, false)
}

/// Duration and start has to be either both set or not set.
/// Details about algorithm: https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning
fn alpha_beta(
    agent: &Agent,
    state: &GameState,
    alpha: i32,
    beta: i32,
    depth: u8,
    timer: Option<&Timer>,
    max: bool,
    first_call: bool, // for debug.
) -> (Option<Action>, i32) {
    // Check search time
    if timer.is_some() {
        let timer = timer.unwrap();
        let now = Instant::now();
        if now.duration_since(timer.start) >= timer.duration {
            debug!("Alpha beta timeout.");
            return (None, utility(agent, state));
        }
    }

    // Check search depth and if game over.
    if depth == 0 || state.game_over.is_some() {
        return (None, utility(agent, state));
    }

    // Get available actions for current state
    // Also store reference to state, to make cmp function work (ugly, I know).
    let mut actions: Vec<(&GameState, Action)> = get_actions(state)
        .iter()
        .map(|a| (state, a.clone()))
        .collect();
    // Best branch/action for current state will be stored here (min or max)
    let mut best_action: Option<Action> = None;
    // Node/state utility will be stored here
    let mut best_utility: i32;

    // Put good actions in the beginning
    actions.sort_by(action_cmp);

    if max {
        let mut new_alpha = alpha;
        best_utility = i32::MIN;
        for (_, action) in actions {
            let (_, utility) = try_branch(
                agent,
                state,
                new_alpha,
                beta,
                depth - 1,
                timer,
                false,
                &action,
            );

            if first_call {
                debug!(
                    "Possible action: Action: {:?}, Utility: {:?}",
                    &action, utility
                );
            }

            // If utility is better than current best, store new value.
            if utility > best_utility {
                best_action = Some(action);
                best_utility = utility;
                if best_utility > new_alpha {
                    new_alpha = best_utility;
                }
            }

            // Minimizer will not allow this branch if value is higher than
            // global maximum (beta). Prune.
            if best_utility >= beta {
                break;
            }
        }
    } else {
        let mut new_beta = beta;
        best_utility = i32::MAX;

        for (_, action) in actions {
            let (_, utility) = try_branch(
                agent,
                &state,
                alpha,
                new_beta,
                depth - 1,
                timer,
                true,
                &action,
            );

            // If utility is better than current best, store new value.
            if utility < best_utility {
                best_action = Some(action); // Will never be used. Remove?
                best_utility = utility;
                if best_utility < new_beta {
                    new_beta = best_utility;
                }
            }

            // Maximizer will not allow this branch if value is lower than
            // global minimum (alpha). Prune.
            if best_utility <= alpha {
                break;
            }
        }
    }

    return (best_action, best_utility);
}

fn alpha_beta_search(agent: &Agent, state: &GameState) -> Option<Action> {
    let mut depth = 4;

    if agent.depth.is_some() {
        depth = agent.depth.unwrap();
    }
    if agent.duration.is_some() {
        let timer = Timer {
            start: Instant::now(),
            duration: agent.duration.unwrap(),
        };
        debug!("Current state utility: {:?}", utility(agent, state));
        let (action, utility) = alpha_beta(
            agent,
            state,
            i32::MIN,
            i32::MAX,
            depth,
            Some(&timer),
            true,
            true,
        );
        if action.is_some() {
            debug!(
                "{:?}: Action: {:?}, Utility: {:?}",
                agent.color,
                action.as_ref().unwrap(),
                utility
            );
        }
        return action;
    }
    debug!("Current state utility: {:?}", utility(agent, state));
    let (action, utility) = alpha_beta(agent, state, i32::MIN, i32::MAX, depth, None, true, true);
    if action.is_some() {
        debug!(
            "{:?}: Action: {:?}, Utility: {:?}",
            agent.color,
            action.as_ref().unwrap(),
            utility
        );
    }
    return action;
}

/// Returns action from super ordinary single threaded Alpha Beta Prune search.
pub fn get_action(agent: &Agent, state: &logic::GameState) -> Option<Action> {
    return alpha_beta_search(agent, state);
}

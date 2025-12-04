use std::cmp::Reverse;

use anyhow::{anyhow, Result};
use nom::{
    branch::alt, bytes::streaming::tag, combinator::map_opt, multi::separated_list1,
    sequence::preceded,
};
use priority_queue::PriorityQueue;

// This is just awful but it works.

type SolverInput = Burrow;

const ROOM_COUNT: usize = 4;
const ROOM_DEPTH: usize = 2;
const SIDE_ROOM_DEPTH: usize = 2;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(Clone, Copy)]
enum BurrowPosition {
    SideRoomLeft,
    OverRoom0,
    HallwayLeft,
    OverRoom1,
    HallwayMiddle,
    OverRoom2,
    HallwayRight,
    OverRoom3,
    SideRoomRight,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Burrow {
    // rooms [0] is the leftmost room, rooms[0][0] is the deepest spot in leftmost room
    rooms: [Vec<Amphipod>; ROOM_COUNT],
    room_depth: usize,
    hallway_left: Option<Amphipod>,
    hallway_middle: Option<Amphipod>,
    hallway_right: Option<Amphipod>,
    // [0] is the shallow spot, [1] is the deep one
    left_side_room: [Option<Amphipod>; SIDE_ROOM_DEPTH],
    right_side_room: [Option<Amphipod>; SIDE_ROOM_DEPTH],
}

impl Amphipod {
    fn from_ascii(text: &[u8]) -> Option<Amphipod> {
        match text {
            b"A" => Some(Amphipod::Amber),
            b"B" => Some(Amphipod::Bronze),
            b"C" => Some(Amphipod::Copper),
            b"D" => Some(Amphipod::Desert),
            _ => None,
        }
    }

    fn desired_room_pos(&self) -> BurrowPosition {
        match self {
            Amphipod::Amber => BurrowPosition::OverRoom0,
            Amphipod::Bronze => BurrowPosition::OverRoom1,
            Amphipod::Copper => BurrowPosition::OverRoom2,
            Amphipod::Desert => BurrowPosition::OverRoom3,
        }
    }

    fn desired_room_idx(&self) -> usize {
        Burrow::room_idx_under_pos(self.desired_room_pos())
    }

    fn move_cost(&self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }
}

impl BurrowPosition {
    fn base_moves_to(&self, to: Self) -> usize {
        match self {
            Self::SideRoomLeft => match to {
                Self::SideRoomLeft => panic!("Getting move cost to the same spot"),
                Self::OverRoom0 => 1,
                Self::HallwayLeft => 2,
                Self::OverRoom1 => 3,
                Self::HallwayMiddle => 4,
                Self::OverRoom2 => 5,
                Self::HallwayRight => 6,
                Self::OverRoom3 => 7,
                Self::SideRoomRight => 8,
            },
            Self::OverRoom0 => match to {
                Self::SideRoomLeft => to.base_moves_to(*self),
                Self::OverRoom0 => panic!("Getting move cost to the same spot"),
                Self::HallwayLeft => 1,
                Self::OverRoom1 => 2,
                Self::HallwayMiddle => 3,
                Self::OverRoom2 => 4,
                Self::HallwayRight => 5,
                Self::OverRoom3 => 6,
                Self::SideRoomRight => 7,
            },
            Self::HallwayLeft => match to {
                Self::SideRoomLeft | Self::OverRoom0 => to.base_moves_to(*self),
                Self::HallwayLeft => panic!("Getting move cost to the same spot"),
                Self::OverRoom1 => 1,
                Self::HallwayMiddle => 2,
                Self::OverRoom2 => 3,
                Self::HallwayRight => 4,
                Self::OverRoom3 => 5,
                Self::SideRoomRight => 6,
            },
            Self::OverRoom1 => match to {
                Self::SideRoomLeft | Self::OverRoom0 | Self::HallwayLeft => to.base_moves_to(*self),
                Self::OverRoom1 => panic!("Getting move cost to the same spot"),
                Self::HallwayMiddle => 1,
                Self::OverRoom2 => 2,
                Self::HallwayRight => 3,
                Self::OverRoom3 => 4,
                Self::SideRoomRight => 5,
            },
            Self::HallwayMiddle => match to {
                Self::SideRoomLeft | Self::OverRoom0 | Self::HallwayLeft | Self::OverRoom1 => {
                    to.base_moves_to(*self)
                }
                Self::HallwayMiddle => panic!("Getting move cost to the same spot"),
                Self::OverRoom2 => 1,
                Self::HallwayRight => 2,
                Self::OverRoom3 => 3,
                Self::SideRoomRight => 4,
            },
            Self::OverRoom2 => match to {
                Self::SideRoomLeft
                | Self::OverRoom0
                | Self::HallwayLeft
                | Self::OverRoom1
                | Self::HallwayMiddle => to.base_moves_to(*self),
                Self::OverRoom2 => panic!("Getting move cost to the same spot"),
                Self::HallwayRight => 1,
                Self::OverRoom3 => 2,
                Self::SideRoomRight => 3,
            },
            Self::HallwayRight => match to {
                Self::SideRoomLeft
                | Self::OverRoom0
                | Self::HallwayLeft
                | Self::OverRoom1
                | Self::HallwayMiddle
                | Self::OverRoom2 => to.base_moves_to(*self),
                Self::HallwayRight => panic!("Getting move cost to the same spot"),
                Self::OverRoom3 => 1,
                Self::SideRoomRight => 2,
            },
            Self::OverRoom3 => match to {
                Self::SideRoomLeft
                | Self::OverRoom0
                | Self::HallwayLeft
                | Self::OverRoom1
                | Self::HallwayMiddle
                | Self::OverRoom2
                | Self::HallwayRight => to.base_moves_to(*self),
                Self::OverRoom3 => panic!("Getting move cost to the same spot"),
                Self::SideRoomRight => 1,
            },
            Self::SideRoomRight => match to {
                Self::SideRoomLeft
                | Self::OverRoom0
                | Self::HallwayLeft
                | Self::OverRoom1
                | Self::HallwayMiddle
                | Self::OverRoom2
                | Self::HallwayRight
                | Self::OverRoom3 => to.base_moves_to(*self),
                Self::SideRoomRight => panic!("Getting move cost to the same spot"),
            },
        }
    }
}

impl Burrow {
    fn pos_over_room_idx(idx: usize) -> BurrowPosition {
        match idx {
            0 => BurrowPosition::OverRoom0,
            1 => BurrowPosition::OverRoom1,
            2 => BurrowPosition::OverRoom2,
            3 => BurrowPosition::OverRoom3,
            _ => panic!("Requesting position over room index out of bounds"),
        }
    }

    fn room_idx_under_pos(pos: BurrowPosition) -> usize {
        match pos {
            BurrowPosition::OverRoom0 => 0,
            BurrowPosition::OverRoom1 => 1,
            BurrowPosition::OverRoom2 => 2,
            BurrowPosition::OverRoom3 => 3,
            _ => panic!("Requesting room under a position that has no room underneath"),
        }
    }

    fn get_room_under_pos(&self, pos: BurrowPosition) -> &Vec<Amphipod> {
        let idx = Self::room_idx_under_pos(pos);
        &self.rooms[idx]
    }

    fn get_room_under_pos_mut(&mut self, pos: BurrowPosition) -> &mut Vec<Amphipod> {
        let idx = match pos {
            BurrowPosition::OverRoom0 => 0,
            BurrowPosition::OverRoom1 => 1,
            BurrowPosition::OverRoom2 => 2,
            BurrowPosition::OverRoom3 => 3,
            _ => panic!("Requesting room under a position that has no room underneath"),
        };
        &mut self.rooms[idx]
    }

    fn room_needs_popping(&self, room_idx: usize) -> bool {
        if let Some(pod) = self.rooms[room_idx].last() {
            if pod.desired_room_idx() != room_idx {
                true
            } else {
                self.rooms[room_idx].iter().any(|p| p != pod)
            }
        } else {
            false
        }
    }

    fn is_solved(&self) -> bool {
        for (ridx, room) in self.rooms.iter().enumerate() {
            if room.len() != self.room_depth {
                return false;
            }
            if room.iter().any(|p| p.desired_room_idx() != ridx) {
                return false;
            }
        }
        true
    }

    fn is_position_occupied(&self, pos: BurrowPosition) -> bool {
        match pos {
            BurrowPosition::SideRoomLeft => self.left_side_room.iter().all(|o| o.is_some()),
            BurrowPosition::HallwayLeft => self.hallway_left.is_some(),
            BurrowPosition::HallwayMiddle => self.hallway_middle.is_some(),
            BurrowPosition::HallwayRight => self.hallway_right.is_some(),
            BurrowPosition::SideRoomRight => self.right_side_room.iter().all(|o| o.is_some()),
            _ => false,
        }
    }

    fn is_position_unoccupied(&self, pos: BurrowPosition) -> bool {
        !self.is_position_occupied(pos)
    }

    fn can_move(&self, from: BurrowPosition, to: BurrowPosition) -> bool {
        match from {
            BurrowPosition::SideRoomLeft => match to {
                BurrowPosition::SideRoomLeft => panic!("Checking move to the same spot"),
                BurrowPosition::OverRoom0 => true,
                BurrowPosition::HallwayLeft | BurrowPosition::OverRoom1 => {
                    self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                }
                BurrowPosition::HallwayMiddle | BurrowPosition::OverRoom2 => {
                    self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                }
                BurrowPosition::HallwayRight | BurrowPosition::OverRoom3 => {
                    self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                        && self.is_position_unoccupied(BurrowPosition::HallwayRight)
                }
                BurrowPosition::SideRoomRight => {
                    self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                        && self.is_position_unoccupied(BurrowPosition::HallwayRight)
                        && self.is_position_unoccupied(BurrowPosition::SideRoomRight)
                }
            },
            BurrowPosition::OverRoom0 => match to {
                BurrowPosition::SideRoomLeft => {
                    self.is_position_unoccupied(BurrowPosition::SideRoomLeft)
                }
                BurrowPosition::OverRoom0 => panic!("Checking move to the same spot"),
                BurrowPosition::HallwayLeft | BurrowPosition::OverRoom1 => {
                    self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                }
                BurrowPosition::HallwayMiddle | BurrowPosition::OverRoom2 => {
                    self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                }
                BurrowPosition::HallwayRight | BurrowPosition::OverRoom3 => {
                    self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                        && self.is_position_unoccupied(BurrowPosition::HallwayRight)
                }
                BurrowPosition::SideRoomRight => {
                    self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                        && self.is_position_unoccupied(BurrowPosition::HallwayRight)
                        && self.is_position_unoccupied(BurrowPosition::SideRoomRight)
                }
            },
            BurrowPosition::HallwayLeft => match to {
                BurrowPosition::SideRoomLeft => {
                    self.is_position_unoccupied(BurrowPosition::SideRoomLeft)
                }
                BurrowPosition::OverRoom0 => true,
                BurrowPosition::HallwayLeft => panic!("Checking move to the same spot"),
                BurrowPosition::OverRoom1 => true,
                BurrowPosition::HallwayMiddle | BurrowPosition::OverRoom2 => {
                    self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                }
                BurrowPosition::HallwayRight | BurrowPosition::OverRoom3 => {
                    self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                        && self.is_position_unoccupied(BurrowPosition::HallwayRight)
                }
                BurrowPosition::SideRoomRight => {
                    self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                        && self.is_position_unoccupied(BurrowPosition::HallwayRight)
                        && self.is_position_unoccupied(BurrowPosition::SideRoomRight)
                }
            },
            BurrowPosition::OverRoom1 => match to {
                BurrowPosition::SideRoomLeft => {
                    self.is_position_unoccupied(BurrowPosition::SideRoomLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                }
                BurrowPosition::OverRoom0 | BurrowPosition::HallwayLeft => {
                    self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                }
                BurrowPosition::OverRoom1 => panic!("Checking move to the same spot"),
                BurrowPosition::HallwayMiddle | BurrowPosition::OverRoom2 => {
                    self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                }
                BurrowPosition::HallwayRight | BurrowPosition::OverRoom3 => {
                    self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                        && self.is_position_unoccupied(BurrowPosition::HallwayRight)
                }
                BurrowPosition::SideRoomRight => {
                    self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                        && self.is_position_unoccupied(BurrowPosition::HallwayRight)
                        && self.is_position_unoccupied(BurrowPosition::SideRoomRight)
                }
            },
            BurrowPosition::HallwayMiddle => match to {
                BurrowPosition::SideRoomLeft => {
                    self.is_position_unoccupied(BurrowPosition::SideRoomLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                }
                BurrowPosition::OverRoom0 | BurrowPosition::HallwayLeft => {
                    self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                }
                BurrowPosition::OverRoom1 => true,
                BurrowPosition::HallwayMiddle => panic!("Checking move to the same spot"),
                BurrowPosition::OverRoom2 => true,
                BurrowPosition::HallwayRight | BurrowPosition::OverRoom3 => {
                    self.is_position_unoccupied(BurrowPosition::HallwayRight)
                }
                BurrowPosition::SideRoomRight => {
                    self.is_position_unoccupied(BurrowPosition::HallwayRight)
                        && self.is_position_unoccupied(BurrowPosition::SideRoomRight)
                }
            },
            BurrowPosition::OverRoom2 => match to {
                BurrowPosition::SideRoomLeft => {
                    self.is_position_unoccupied(BurrowPosition::SideRoomLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                }
                BurrowPosition::OverRoom0 | BurrowPosition::HallwayLeft => {
                    self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                }
                BurrowPosition::OverRoom1 | BurrowPosition::HallwayMiddle => {
                    self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                }
                BurrowPosition::OverRoom2 => panic!("Checking move to the same spot"),
                BurrowPosition::HallwayRight | BurrowPosition::OverRoom3 => {
                    self.is_position_unoccupied(BurrowPosition::HallwayRight)
                }
                BurrowPosition::SideRoomRight => {
                    self.is_position_unoccupied(BurrowPosition::HallwayRight)
                        && self.is_position_unoccupied(BurrowPosition::SideRoomRight)
                }
            },
            BurrowPosition::HallwayRight => match to {
                BurrowPosition::SideRoomLeft => {
                    self.is_position_unoccupied(BurrowPosition::SideRoomLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                }
                BurrowPosition::OverRoom0 | BurrowPosition::HallwayLeft => {
                    self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                }
                BurrowPosition::OverRoom1 | BurrowPosition::HallwayMiddle => {
                    self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                }
                BurrowPosition::OverRoom2 => true,
                BurrowPosition::HallwayRight => panic!("Checking move to the same spot"),
                BurrowPosition::OverRoom3 => true,
                BurrowPosition::SideRoomRight => {
                    self.is_position_unoccupied(BurrowPosition::SideRoomRight)
                }
            },
            BurrowPosition::OverRoom3 => match to {
                BurrowPosition::SideRoomLeft => {
                    self.is_position_unoccupied(BurrowPosition::SideRoomLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                        && self.is_position_unoccupied(BurrowPosition::HallwayRight)
                }
                BurrowPosition::OverRoom0 | BurrowPosition::HallwayLeft => {
                    self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                        && self.is_position_unoccupied(BurrowPosition::HallwayRight)
                }
                BurrowPosition::OverRoom1 | BurrowPosition::HallwayMiddle => {
                    self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                        && self.is_position_unoccupied(BurrowPosition::HallwayRight)
                }
                BurrowPosition::OverRoom2 | BurrowPosition::HallwayRight => {
                    self.is_position_unoccupied(BurrowPosition::HallwayRight)
                }
                BurrowPosition::OverRoom3 => panic!("Checking move to the same spot"),
                BurrowPosition::SideRoomRight => {
                    self.is_position_unoccupied(BurrowPosition::SideRoomRight)
                }
            },
            BurrowPosition::SideRoomRight => match to {
                BurrowPosition::SideRoomLeft => {
                    self.is_position_unoccupied(BurrowPosition::SideRoomLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                        && self.is_position_unoccupied(BurrowPosition::HallwayRight)
                }
                BurrowPosition::OverRoom0 | BurrowPosition::HallwayLeft => {
                    self.is_position_unoccupied(BurrowPosition::HallwayLeft)
                        && self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                        && self.is_position_unoccupied(BurrowPosition::HallwayRight)
                }
                BurrowPosition::OverRoom1 | BurrowPosition::HallwayMiddle => {
                    self.is_position_unoccupied(BurrowPosition::HallwayMiddle)
                        && self.is_position_unoccupied(BurrowPosition::HallwayRight)
                }
                BurrowPosition::OverRoom2 | BurrowPosition::HallwayRight => {
                    self.is_position_unoccupied(BurrowPosition::HallwayRight)
                }
                BurrowPosition::OverRoom3 => true,
                BurrowPosition::SideRoomRight => panic!("Checking move to the same spot"),
            },
        }
    }
}

fn queue_move_into_room(
    state: Burrow,
    pod: Amphipod,
    from: BurrowPosition,
    current_cost: usize,
    known_states: &mut PriorityQueue<Burrow, Reverse<usize>>,
) {
    let mut state = state;
    let desired_room_pos = pod.desired_room_pos();
    let can_move_over = state.can_move(from, desired_room_pos);
    let desired_room = state.get_room_under_pos(desired_room_pos);
    let can_move_into = desired_room.iter().all(|o| *o == pod);
    if can_move_over && can_move_into {
        let cost = {
            let tiles_moved_into_room = state.room_depth - desired_room.len();
            let tiles_moved = from.base_moves_to(desired_room_pos) + tiles_moved_into_room;
            current_cost + (tiles_moved * pod.move_cost())
        };
        state.get_room_under_pos_mut(desired_room_pos).push(pod);
        known_states.push_increase(state, Reverse(cost));
    }
}

fn queue_moves_from_room(
    state: Burrow,
    current_cost: usize,
    room_idx: usize,
    known_states: &mut PriorityQueue<Burrow, Reverse<usize>>,
) {
    let mut state = state;
    let pod = state.rooms[room_idx].pop().unwrap();
    let start_pos = Burrow::pos_over_room_idx(room_idx);
    let leave_room_moves = state.room_depth - state.rooms[room_idx].len();

    for end_pos in &[
        BurrowPosition::HallwayLeft,
        BurrowPosition::HallwayMiddle,
        BurrowPosition::HallwayRight,
    ] {
        if !state.can_move(start_pos, *end_pos) {
            continue;
        }
        let cost = {
            let tiles_moved = start_pos.base_moves_to(*end_pos) + leave_room_moves;
            current_cost + (tiles_moved * pod.move_cost())
        };
        let mut new_state = state.clone();
        match end_pos {
            BurrowPosition::HallwayLeft => new_state.hallway_left = Some(pod),
            BurrowPosition::HallwayMiddle => new_state.hallway_middle = Some(pod),
            BurrowPosition::HallwayRight => new_state.hallway_right = Some(pod),
            _ => panic!("Invalid end pos in queue moves from room"),
        }
        known_states.push_increase(new_state, Reverse(cost));
    }

    if state.left_side_room[0].is_none() && state.can_move(start_pos, BurrowPosition::SideRoomLeft)
    {
        // shallow
        {
            let cost = {
                let tiles_moved =
                    start_pos.base_moves_to(BurrowPosition::SideRoomLeft) + leave_room_moves;
                current_cost + (tiles_moved * pod.move_cost())
            };
            let mut new_state = state.clone();
            new_state.left_side_room[0] = Some(pod);
            known_states.push_increase(new_state, Reverse(cost));
        }
        // deep
        if state.left_side_room[1].is_none() {
            let cost = {
                let tiles_moved =
                    start_pos.base_moves_to(BurrowPosition::SideRoomLeft) + leave_room_moves + 1;
                current_cost + (tiles_moved * pod.move_cost())
            };
            let mut new_state = state.clone();
            new_state.left_side_room[1] = Some(pod);
            known_states.push_increase(new_state, Reverse(cost));
        }
    }

    if state.right_side_room[0].is_none()
        && state.can_move(start_pos, BurrowPosition::SideRoomRight)
    {
        // shallow
        {
            let cost = {
                let tiles_moved =
                    start_pos.base_moves_to(BurrowPosition::SideRoomRight) + leave_room_moves;
                current_cost + (tiles_moved * pod.move_cost())
            };
            let mut new_state = state.clone();
            new_state.right_side_room[0] = Some(pod);
            known_states.push_increase(new_state, Reverse(cost));
        }
        // deep
        if state.right_side_room[1].is_none() {
            let cost = {
                let tiles_moved =
                    start_pos.base_moves_to(BurrowPosition::SideRoomRight) + leave_room_moves + 1;
                current_cost + (tiles_moved * pod.move_cost())
            };
            let mut new_state = state.clone();
            new_state.right_side_room[1] = Some(pod);
            known_states.push_increase(new_state, Reverse(cost));
        }
    }
}

fn queue_possible_moves(
    state: Burrow,
    current_cost: usize,
    known_states: &mut PriorityQueue<Burrow, Reverse<usize>>,
) {
    // from left side room
    if let Some(pod) = state.left_side_room[0] {
        // leaving shallow left side room
        let mut new_state = state.clone();
        new_state.left_side_room[0].take();
        queue_move_into_room(
            new_state,
            pod,
            BurrowPosition::SideRoomLeft,
            current_cost,
            known_states,
        );
    } else if let Some(pod) = state.left_side_room[1] {
        // leaving deep left side room
        let mut new_state = state.clone();
        new_state.left_side_room[1].take();
        queue_move_into_room(
            new_state,
            pod,
            BurrowPosition::SideRoomLeft,
            current_cost + pod.move_cost(),
            known_states,
        );
    }
    // from hallway
    if let Some(pod) = state.hallway_left {
        let mut new_state = state.clone();
        new_state.hallway_left.take();
        queue_move_into_room(
            new_state,
            pod,
            BurrowPosition::HallwayLeft,
            current_cost,
            known_states,
        );
    }
    if let Some(pod) = state.hallway_middle {
        let mut new_state = state.clone();
        new_state.hallway_middle.take();
        queue_move_into_room(
            new_state,
            pod,
            BurrowPosition::HallwayMiddle,
            current_cost,
            known_states,
        );
    }
    if let Some(pod) = state.hallway_right {
        let mut new_state = state.clone();
        new_state.hallway_right.take();
        queue_move_into_room(
            new_state,
            pod,
            BurrowPosition::HallwayRight,
            current_cost,
            known_states,
        );
    }
    // from right side room
    if let Some(pod) = state.right_side_room[0] {
        // leaving shallow right side room
        let mut new_state = state.clone();
        new_state.right_side_room[0].take();
        queue_move_into_room(
            new_state,
            pod,
            BurrowPosition::SideRoomRight,
            current_cost,
            known_states,
        );
    } else if let Some(pod) = state.right_side_room[1] {
        // leaving deep right side room
        let mut new_state = state.clone();
        new_state.right_side_room[1].take();
        queue_move_into_room(
            new_state,
            pod,
            BurrowPosition::SideRoomRight,
            current_cost + pod.move_cost(),
            known_states,
        );
    }

    for room_idx in 0..ROOM_COUNT {
        if state.room_needs_popping(room_idx) {
            queue_moves_from_room(state.clone(), current_cost, room_idx, known_states);
        }
    }
}

fn calc_solve_cost(burrow: Burrow) -> usize {
    let mut states = PriorityQueue::new();
    states.push(burrow, Reverse(0));
    while let Some((state, cost)) = states.pop() {
        if state.is_solved() {
            return cost.0;
        }
        queue_possible_moves(state, cost.0, &mut states);
    }

    0
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    let amphipods = {
        let prefix = tag(b"#############\n#...........#\n###");
        let line_sep = tag(b"###\n  #");
        let room_sep = tag(b"#");
        let abcd_alt = alt((tag(b"A"), tag(b"B"), tag(b"C"), tag(b"D")));
        let abcd = map_opt(abcd_alt, Amphipod::from_ascii);
        let line_rooms = separated_list1(room_sep, abcd);
        let rooms = separated_list1::<_, _, _, nom::error::Error<_>, _, _>(line_sep, line_rooms);
        preceded(prefix, rooms)(file)
    }
    .map_err(|_| anyhow!("Failed parsing amphipods"))?
    .1
    .into_iter()
    .flatten();

    let rooms = {
        let mut rooms = [vec![], vec![], vec![], vec![]];
        let mut room_it = 0;
        for pod in amphipods.into_iter().rev() {
            rooms[ROOM_COUNT - room_it - 1].push(pod);
            room_it = (room_it + 1) % ROOM_COUNT;
        }
        rooms
    };

    Ok(Burrow {
        rooms,
        room_depth: ROOM_DEPTH,
        hallway_left: None,
        hallway_middle: None,
        hallway_right: None,
        left_side_room: [None; SIDE_ROOM_DEPTH],
        right_side_room: [None; SIDE_ROOM_DEPTH],
    })
}

pub fn solve_part1(input: &SolverInput) -> usize {
    calc_solve_cost(input.clone())
}

pub fn solve_part2(input: &SolverInput) -> usize {
    let mut state = input.clone();
    state.room_depth += 2;
    state.rooms[0].insert(1, Amphipod::Desert);
    state.rooms[0].insert(1, Amphipod::Desert);
    state.rooms[1].insert(1, Amphipod::Copper);
    state.rooms[1].insert(1, Amphipod::Bronze);
    state.rooms[2].insert(1, Amphipod::Bronze);
    state.rooms[2].insert(1, Amphipod::Amber);
    state.rooms[3].insert(1, Amphipod::Amber);
    state.rooms[3].insert(1, Amphipod::Copper);

    calc_solve_cost(state)
}

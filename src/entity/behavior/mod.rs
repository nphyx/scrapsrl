/**
 * Behaviors for NPCs
 */
use rand::prelude::*;
use tcod::Map;
use crate::entity::{Entity, NPC, Coord};
use crate::game_state::GameState;
use crate::util::plan;

pub trait Behavior {
  fn execute(&self, ent: &mut NPC, state: &GameState, map: &Map);
}

fn move_brownian(ent: &mut NPC, state: &GameState, map: &Map) -> bool {
  let pos = ent.pos();
  let mut rng = rand::thread_rng();
  let mut tries: i8 = 0;
  while tries < 10 {
    let to = Coord{
      x: rng.gen_range(pos.x - 1, pos.x + 2),
      y: rng.gen_range(pos.y - 1, pos.y + 2)
    };
    match plan(&to, &map) {
      Some(coord) => {
        ent.set_pos(coord);
        return true;
      },
      None => {}
    }
    tries += 1;
  }
  return false;
}

pub enum MovementBehavior {
  BrownianWalk
}

impl Behavior for MovementBehavior {
  fn execute(&self, ent: &mut NPC, state: &GameState, map: &Map) {
    match self {
      MovementBehavior::BrownianWalk => { move_brownian(ent, state, map); }
    }
  }
}

use std::collections::HashSet;
use tcod::{Console, BackgroundFlag};
use tcod::colors::Color;

use crate::display::DrawSelf;
use crate::entity::{Entity, Coord};
use crate::game_state::GameState;
use super::entity_part::{
  EntityComponent,
  EntityComponentVariety,
  EntityComponentSide
};

#[derive(Hash, Eq, PartialEq)]
pub enum VitalMod {
  Stamina,
  Focus,
  Grit 
}

pub struct Vital {
  buffs: HashSet<VitalMod>,
  debuffs: HashSet<VitalMod>
}

pub struct Capability {
}

pub struct Feature {
  capabilities: Vec<Capability>
}

pub struct Item {
}

pub struct Skill {
}

pub struct Character {
  pos: Coord,
  move_plan: Coord,
  ch: char,
  color: Color,

  body: u8,
  mind: u8,
  soul: u8,

  strength: u8,
  grace: u8,
  toughness: u8,

  intellect: u8,
  wits: u8,
  resolve: u8,

  charisma: u8,
  empathy: u8,
  will: u8,

  cur_stamina: u8,
  cur_focus: u8,
  cur_grit: u8,

  skills: Vec<Skill>,
  features: Vec<Feature>,
  body_layout: EntityComponent,
  gear: Vec<Item>,
  vitals: Vec<Vital>
}

impl Character {
  pub fn blank() -> Character {
    Character{
      ch: '?',
      pos: Coord{x:0, y:0},
      color: Color{r: 255, g: 255, b: 255},
      move_plan: Coord{x:0, y:0},
      body: 3, mind: 3, soul: 3,
      strength: 1, grace: 2, toughness: 3,
      intellect: 1, wits: 2, resolve: 3,
      charisma: 1, empathy: 2, will: 3,
      cur_stamina: 1, cur_focus: 1, cur_grit: 1,
      skills: Vec::new(),
      features: Vec::new(),
      body_layout: EntityComponent::new(EntityComponentVariety::Torso, EntityComponentSide::NoSide),
      gear: Vec::new(),
      vitals: Vec::new()}
  }

  pub fn set_ch(&mut self, ch: char) { self.ch = ch; }
  pub fn set_color(&mut self, color: Color) { self.color = color; }

  pub fn get_body_layout(&self) -> &EntityComponent {
    &self.body_layout
  }
  pub fn set_body_layout(&mut self, layout: EntityComponent) {
    self.body_layout = layout;
  }

  pub fn move_plan(&self) -> Coord { self.move_plan }
  pub fn set_move_plan(&mut self, pos: Coord) { self.pos = pos }

  pub fn body(&self) -> u8 { self.body }
  pub fn set_body(&mut self, val:u8) { self.body = val }
  pub fn mind(&self) -> u8 { self.mind }
  pub fn set_mind(&mut self, val:u8) { self.mind = val }
  pub fn soul(&self) -> u8 { self.soul }

  pub fn intellect(&self) -> u8 { self.intellect }
  pub fn set_intellect(&mut self, val:u8) { self.intellect = val }
  pub fn wits(&self) -> u8 { self.wits }
  pub fn set_wits(&mut self, val:u8) { self.wits = val }
  pub fn resolve(&self) -> u8 { self.resolve }
  pub fn set_resolve(&mut self, val:u8) { self.resolve = val }

  pub fn strength(&self) -> u8 { self.strength }
  pub fn set_strength(&mut self, val:u8) { self.strength = val }
  pub fn grace(&self) -> u8 { self.grace }
  pub fn set_grace(&mut self, val:u8) { self.grace = val }
  pub fn toughness(&self) -> u8 { self.toughness }

  pub fn charisma(&self) -> u8 { self.charisma }
  pub fn set_charisma(&mut self, val:u8) { self.charisma = val }
  pub fn empathy(&self) -> u8 { self.empathy }
  pub fn set_empathy(&mut self, val:u8) { self.empathy = val }
  pub fn will(&self) -> u8 { self.will }
  pub fn set_will(&mut self, val:u8) { self.will = val }

  fn compute_vital_modifier(&self, vital_type: VitalMod) -> u8 {
    let mut sum: u8 = 0;
    for vital in self.vitals.iter() {
      if vital.debuffs.contains(&vital_type) { sum += 1; }
      if vital.buffs.contains(&vital_type) { sum += 1; }
    }
    sum
  }

  fn compute_stamina(&self) -> u8 { self.body + self.toughness }
  pub fn stamina(&self) -> (u8, u8, u8) {
    (
      self.compute_stamina(),
      self.cur_stamina,
      self.compute_vital_modifier(VitalMod::Stamina)
    )
  }
  pub fn spend_stamina(&mut self, amt: u8) -> bool {
    if amt <= self.cur_stamina {
      self.cur_stamina -= amt;
      return true;
    }
    return false;
  }

  fn compute_focus(&self) -> u8 { self.mind + self.resolve }
  pub fn focus(&self) -> (u8, u8, u8) {
    (
      self.compute_focus(),
      self.cur_focus,
      self.compute_vital_modifier(VitalMod::Focus)
    )
  }
  pub fn spend_focus(&mut self, amt: u8) -> bool {
    if amt >= self.cur_focus {
      self.cur_focus -= amt;
      return true;
    }
    return false;
  }

  fn compute_grit(&self) -> u8 { self.soul + self.will }
  pub fn grit(&self) -> (u8, u8, u8) {
    (
      self.compute_grit(),
      self.cur_grit,
      self.compute_vital_modifier(VitalMod::Focus)
    )
  }
  pub fn spend_grit(&mut self, amt: u8) -> bool {
    if amt >= self.cur_grit {
      self.cur_grit -= amt;
      return true;
    }
    return false;
  }
  pub fn tick(&mut self) {
    if self.cur_stamina < self.compute_stamina() {
      self.cur_stamina += 1;
    }
    if self.cur_focus < self.compute_focus() {
      self.cur_focus += 1;
    }
    if self.cur_grit < self.compute_grit() {
      self.cur_grit += 1;
    }
  }
}

impl Entity for Character {
  fn pos(&self) -> Coord { self.pos }
  fn set_pos(&mut self, pos: Coord) { self.pos = pos }
  fn tick(&mut self, state: &GameState) {
  }
}

impl DrawSelf for Character {
  fn draw(&self, console: &mut Console) {
    console.put_char(self.pos().x, self.pos().y, self.ch, BackgroundFlag::None);
    console.set_char_foreground(self.pos().x, self.pos().y, self.color);
  }
}

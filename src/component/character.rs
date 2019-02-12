use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum VitalMod {
    Stamina,
    Focus,
    Grit,
}

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize)]
pub struct Character {
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
}

impl Component for Character {
    type Storage = VecStorage<Self>;
}

impl Character {
    pub fn blank() -> Character {
        Character {
            body: 3,
            mind: 3,
            soul: 3,
            strength: 1,
            grace: 2,
            toughness: 3,
            intellect: 1,
            wits: 2,
            resolve: 3,
            charisma: 1,
            empathy: 2,
            will: 3,
            cur_stamina: 1,
            cur_focus: 1,
            cur_grit: 1,
        }
    }
    pub fn body(&self) -> u8 {
        self.body
    }
    pub fn mind(&self) -> u8 {
        self.mind
    }
    pub fn soul(&self) -> u8 {
        self.soul
    }
    pub fn intellect(&self) -> u8 {
        self.intellect
    }
    pub fn wits(&self) -> u8 {
        self.wits
    }
    pub fn resolve(&self) -> u8 {
        self.resolve
    }
    pub fn strength(&self) -> u8 {
        self.strength
    }
    pub fn grace(&self) -> u8 {
        self.grace
    }
    pub fn toughness(&self) -> u8 {
        self.toughness
    }
    pub fn charisma(&self) -> u8 {
        self.charisma
    }
    pub fn empathy(&self) -> u8 {
        self.empathy
    }
    pub fn will(&self) -> u8 {
        self.will
    }

    fn compute_focus(&self) -> u8 {
        self.mind + self.resolve
    }
    pub fn focus(&self) -> (u8, u8, u8) {
        (
            self.compute_focus(),
            self.cur_focus,
            self.compute_vital_modifier(VitalMod::Focus),
        )
    }

    fn compute_vital_modifier(&self, _vital_type: VitalMod) -> u8 {
        0
        /* TODO implement me
          let mut sum: u8 = 0;
          for vital in self.vitals.iter() {
            if vital.debuffs.contains(&vital_type) { sum += 1; }
            if vital.buffs.contains(&vital_type) { sum += 1; }
          }
          sum
        */
    }

    fn compute_stamina(&self) -> u8 {
        self.body + self.toughness
    }
    pub fn stamina(&self) -> (u8, u8, u8) {
        (
            self.compute_stamina(),
            self.cur_stamina,
            self.compute_vital_modifier(VitalMod::Stamina),
        )
    }

    fn compute_grit(&self) -> u8 {
        self.soul + self.will
    }
    pub fn grit(&self) -> (u8, u8, u8) {
        (
            self.compute_grit(),
            self.cur_grit,
            self.compute_vital_modifier(VitalMod::Focus),
        )
    }

    /* TODO use when these are needed
    pub fn set_body(&mut self, val:u8) { self.body = val }
    pub fn set_mind(&mut self, val:u8) { self.mind = val }

    pub fn set_intellect(&mut self, val:u8) { self.intellect = val }
    pub fn set_wits(&mut self, val:u8) { self.wits = val }
    pub fn set_resolve(&mut self, val:u8) { self.resolve = val }

    pub fn set_strength(&mut self, val:u8) { self.strength = val }
    pub fn set_grace(&mut self, val:u8) { self.grace = val }

    pub fn set_charisma(&mut self, val:u8) { self.charisma = val }
    pub fn set_empathy(&mut self, val:u8) { self.empathy = val }
    pub fn set_will(&mut self, val:u8) { self.will = val }
    pub fn spend_stamina(&mut self, amt: u8) -> bool {
      if amt <= self.cur_stamina {
        self.cur_stamina -= amt;
        return true;
      }
      return false;
    }

    pub fn spend_focus(&mut self, amt: u8) -> bool {
      if amt >= self.cur_focus {
        self.cur_focus -= amt;
        return true;
      }
      return false;
    }
    pub fn spend_grit(&mut self, amt: u8) -> bool {
      if amt >= self.cur_grit {
        self.cur_grit -= amt;
        return true;
      }
      return false;
    }
    */
}

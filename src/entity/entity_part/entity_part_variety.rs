#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum EntityComponentVariety {
  Head,
  Eye,
  Nose,
  Ear,
  Mouth,
  Torso,
  Arm,
  Hand,
  Hips,
  Leg,
  Foot,
  Stalk,
  Tentacle,
  Pseudopod,
  Carapace,
  Shell,
  Hoof,
  Claw,
  Pincer,
  Mandible,
  Antenna,
  Thorax,
  Abdomen
}

impl EntityComponentVariety {
  pub fn to_string(&self) -> &'static str {
    match self {
      EntityComponentVariety::Head => "head",
      EntityComponentVariety::Eye => "eye",
      EntityComponentVariety::Nose => "nose",
      EntityComponentVariety::Ear => "ear",
      EntityComponentVariety::Mouth => "mouth",
      EntityComponentVariety::Torso => "torso",
      EntityComponentVariety::Arm => "arm",
      EntityComponentVariety::Hand => "hand",
      EntityComponentVariety::Hips => "hips",
      EntityComponentVariety::Leg => "leg",
      EntityComponentVariety::Foot => "foot",
      EntityComponentVariety::Stalk => "stalk",
      EntityComponentVariety::Tentacle => "tentacle",
      EntityComponentVariety::Pseudopod => "pseudopod",
      EntityComponentVariety::Carapace => "carapace",
      EntityComponentVariety::Shell => "shell",
      EntityComponentVariety::Hoof => "hoof",
      EntityComponentVariety::Claw => "claw",
      EntityComponentVariety::Pincer => "pincer",
      EntityComponentVariety::Mandible => "mandible",
      EntityComponentVariety::Antenna => "antenna",
      EntityComponentVariety::Thorax => "thorax",
      EntityComponentVariety::Abdomen => "abdomen"
    }
  }
}

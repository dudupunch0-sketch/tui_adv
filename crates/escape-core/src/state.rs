#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayerState {
    pub health: i32,
    pub sanity: i32,
    pub battery: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameState {
    pub seed: u64,
    pub turn: u32,
    pub location_id: String,
    pub player: PlayerState,
    pub flags: Vec<String>,
    pub clues: Vec<String>,
}

impl GameState {
    pub fn new_printer_scene(seed: u64) -> Self {
        Self {
            seed,
            turn: 0,
            location_id: "printer_area".to_string(),
            player: PlayerState {
                health: 92,
                sanity: 67,
                battery: 41,
            },
            flags: Vec::new(),
            clues: Vec::new(),
        }
    }

    pub(crate) fn add_flag_once(&mut self, flag: &str) {
        if !self.flags.iter().any(|existing| existing == flag) {
            self.flags.push(flag.to_string());
        }
    }

    pub(crate) fn add_clue_once(&mut self, clue: &str) {
        if !self.clues.iter().any(|existing| existing == clue) {
            self.clues.push(clue.to_string());
        }
    }
}

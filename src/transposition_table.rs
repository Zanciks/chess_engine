use std::collections::HashMap;
use chess::ChessMove;

const MAX_SIZE_MB: usize = 512;
const TT_ENTRY_SIZE_BYTES: usize = std::mem::size_of::<TTEntry>();
const MAX_ENTRIES: usize = (MAX_SIZE_MB * 1024 * 1024) / TT_ENTRY_SIZE_BYTES;

#[derive(Clone, Copy)]
pub struct TTEntry {
    depth: u8,
    mv: ChessMove,
    eval: i32,
}

impl TTEntry {
    pub fn new() -> Self {
        TTEntry {
            depth: 0,
            mv: ChessMove::default(), // a1a1
            eval: 0,
        }
    }
    pub fn depth(&self) -> u8 {
        self.depth
    }
    pub fn get_move(&self) -> ChessMove {
        self.mv
    }
    pub fn eval(&self) -> i32 {
        self.eval
    }
    pub fn save(&mut self, depth: u8, mv: ChessMove, eval: i32) {
        self.depth = depth;
        self.mv = mv;
        self.eval = eval;
    }
}

pub struct TranspositionTable {
    entries: HashMap<u64, TTEntry>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable {
            entries: HashMap::with_capacity(MAX_ENTRIES),
        }
    }

    pub fn probe(&self, key: u64) -> Option<&TTEntry> {
        self.entries.get(&key)
    }

    pub fn store(&mut self, key: u64, depth: u8, mv: ChessMove, eval: i32) {
        let mut entry = TTEntry::new();
        entry.save(depth, mv, eval);
        self.entries.insert(key, entry);
    }
}

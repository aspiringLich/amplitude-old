use std::collections::{HashSet, HashMap};



pub struct ParseContext {
    pub item_ids: HashSet<String>,
}

pub struct Track {
    
}

pub struct ParseOutput {
    pub tracks: HashMap<String, Track>
}
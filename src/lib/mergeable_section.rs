use crate::section_fragment;
#[derive(Clone)]
pub struct MergeableSec {
    fragments: Vec<Option<section_fragment::SectionFragment>>,
}

impl MergeableSec {
    pub fn new() -> Self {
        MergeableSec { fragments: vec![] }
    }
}

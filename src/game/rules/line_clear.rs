use crate::app::ImDraw;
use crate::game::playfield::Playfield;

use super::*;

// https://tetris.fandom.com/wiki/Line_clear
#[derive(Copy, Clone, Debug, ImDraw)]
pub enum LineClearRule {
    Naive,
    Sticky,
    Cascade,
}

#[derive(Copy, Clone, Debug, ImDraw)]
pub enum LineClearAnimationType {
    Classic,
}

impl Rules {
    // @TODO is this really a try? I think it's always called correctly
    //pub fn try_clear_lines(&self, playfield: &mut Playfield) -> Option<LineClear> {
    pub fn try_clear_lines(&self, playfield: &mut Playfield) -> bool {
        match self.line_clear_rule {
            LineClearRule::Naive => playfield.try_clear_lines_naive(),
            _ => unimplemented!("line clear rule not implemented!"),
        }
    }
}

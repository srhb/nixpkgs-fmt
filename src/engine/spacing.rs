use rnix::SyntaxElement;

use crate::{
    dsl::{SpaceLoc, SpaceValue, SpacingRule},
    engine::{BlockPosition, FmtModel, SpaceBlock},
    tree_utils::has_newline,
};

impl SpacingRule {
    pub(super) fn apply(&self, element: &SyntaxElement, model: &mut FmtModel) {
        if !self.pattern.matches(element) {
            return;
        }
        if self.space.loc.is_before() {
            let block = model.block_for(element, BlockPosition::Before);
            ensure_space(element, block, self.space.value);
        }
        if self.space.loc.is_after() {
            let block = model.block_for(element, BlockPosition::After);
            ensure_space(element, block, self.space.value);
        }
    }
}

impl SpaceLoc {
    fn is_before(self) -> bool {
        match self {
            SpaceLoc::Before | SpaceLoc::Around => true,
            SpaceLoc::After => false,
        }
    }
    fn is_after(self) -> bool {
        match self {
            SpaceLoc::After | SpaceLoc::Around => true,
            SpaceLoc::Before => false,
        }
    }
}

fn ensure_space(element: &SyntaxElement, block: &mut SpaceBlock, value: SpaceValue) {
    match value {
        SpaceValue::Single => block.set_text(" ", None),
        SpaceValue::SingleOptionalNewline => {
            if !block.has_newline() {
                block.set_text(" ", None)
            }
        }
        SpaceValue::Newline => block.set_text("\n", None),
        SpaceValue::None => block.set_text("", None),
        SpaceValue::NoneOptionalNewline => {
            if !block.has_newline() {
                block.set_text("", None)
            }
        }
        SpaceValue::SingleOrNewline => {
            let parent_is_multiline = element.parent().map_or(false, |it| has_newline(&it));
            if parent_is_multiline {
                block.set_line_break_preserving_existing_newlines(None)
            } else {
                block.set_text(" ", None)
            }
        }
        SpaceValue::NoneOrNewline => {
            let parent_is_multiline = element.parent().map_or(false, |it| has_newline(&it));
            if parent_is_multiline {
                block.set_line_break_preserving_existing_newlines(None)
            } else {
                block.set_text("", None)
            }
        }
    }
}

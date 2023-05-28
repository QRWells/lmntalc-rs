use crate::codegen::Target;

#[derive(Debug, Clone)]
pub enum IL {
    Spec,
    Commit,
    NewAtom(usize /* atom id */),
    RemoveAtom(usize /* atom id */),
    FreeAtom(usize /* atom id */),

    NewLink(
        usize, /* link id */
        usize, /* atom id */
        usize, /* atom id */
    ),
    ReLink(
        usize, /* link id */
        usize, /* atom id */
        usize, /* atom id */
    ),
}

impl IL {
    pub fn to_target(&self, target: Target) -> String {
        todo!("IL::to_target")
    }
}

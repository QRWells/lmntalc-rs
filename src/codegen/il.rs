use crate::codegen::Target;

#[derive(Debug, Clone)]
pub enum IL {
    Spec(usize /* formals */, usize /* locals */),
    Commit(&'static str, usize /* line number */),
    NewAtom(usize /* atom id */, usize /* membrane id */, &'static str),
    RemoveAtom(usize /* atom id */),
    FreeAtom(usize /* atom id */),

    NewLink(
        usize, /* atom 1 id */
        usize, /* pos 1 id */
        usize, /* atom 2 id */
        usize, /* pos 2 id */
        usize, /* mem id */
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

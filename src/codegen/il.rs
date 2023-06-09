use std::fmt::Display;

use crate::parser::data::Link;

#[derive(Debug, Clone)]
pub enum Label {
    RuleSet(usize),
    Rule(usize),
}

#[derive(Debug, Clone)]
pub enum IL {
    /// NewAtom(atom_id, mem_id, name)
    ///
    /// Creates a new atom with the given id, in the given membrane, with the given name and arity.
    NewAtom(
        usize,  /* atom id */
        usize,  /* membrane id */
        String, /* name_funtor */
        usize,  /* arity */
    ),

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

    NewMem(usize /* mem id */, usize /* parent mem id */),
    SetMemName(usize /* mem id */, String /* name */),

    FindAtom(
        usize,  /* to register */
        usize,  /* mem id */
        String, /* name */
        usize,  /* arity */
    ),
    DerefAtom(
        usize, /* to register */
        usize, /* from register */
        usize, /* position */
    ),
    RemoveAtom(usize /* register id */, usize /* parent mem id */),
    FreeAtom(usize /* register id */),

    AnyMem(
        usize,          /* register id */
        usize,          /* parent mem id */
        usize,          /* mem type */
        Option<String>, /* name */
    ),
    NAtoms(usize /* register id */, usize /* count */),
    NMems(usize /* register id */, usize /* count */),
    NoRules(usize /* register id */),
    RemoveMem(usize /* register id */, usize /* parent mem id */),
    FreeMem(usize /* register id */),

    IsInt(usize /* register id */),
    IsFloat(usize /* register id */),
    IsUnary(usize /* register id */),
    IsGround(usize /* register id */),

    Label(Label),
}

impl Display for IL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IL::NewAtom(atom_id, mem_id, name, arity) => {
                write!(f, "new_atom\t{}, {}, {}, {}", atom_id, mem_id, name, arity)
            }
            IL::NewLink(atom1_id, pos1_id, atom2_id, pos2_id, mem_id) => write!(
                f,
                "new_link\t{}, {}, {}, {}, {}",
                atom1_id, pos1_id, atom2_id, pos2_id, mem_id
            ),
            IL::ReLink(link_id, atom1_id, atom2_id) => {
                write!(f, "relink\t{}, {}, {}", link_id, atom1_id, atom2_id)
            }
            IL::NewMem(mem_id, parent_mem_id) => {
                write!(f, "new_mem \t{}, {}", mem_id, parent_mem_id)
            }
            IL::SetMemName(mem_id, name) => write!(f, "set_mem_name\t{}, {}", mem_id, name),
            IL::FindAtom(to_register, mem_id, name, arity) => write!(
                f,
                "find_atom\t{}, {}, {}, {}",
                to_register, mem_id, name, arity
            ),
            IL::DerefAtom(to_register, from_register, position) => write!(
                f,
                "deref_atom\t{}, {}, {}",
                to_register, from_register, position
            ),
            IL::RemoveAtom(register_id, parent_mem_id) => {
                write!(f, "remove_atom\t{}, {}", register_id, parent_mem_id)
            }
            IL::FreeAtom(register_id) => write!(f, "free_atom\t{}", register_id),
            IL::AnyMem(register_id, parent_mem_id, mem_type, name) => write!(
                f,
                "any_mem \t{}, {}, {}, {}",
                register_id,
                parent_mem_id,
                mem_type,
                name.as_ref().unwrap_or(&"".to_owned())
            ),
            IL::NAtoms(register_id, count) => write!(f, "natoms  \t{}, {}", register_id, count),
            IL::NMems(register_id, count) => write!(f, "nmems   \t{}, {}", register_id, count),
            IL::NoRules(_) => todo!(),
            IL::RemoveMem(register_id, mem) => write!(f, "remove_mem\t{}, {}", register_id, mem),
            IL::FreeMem(_) => todo!(),
            IL::Label(l) => match l {
                Label::RuleSet(id) => write!(f, "rule_set\t{}", id),
                Label::Rule(id) => write!(f, "rule\t{}", id),
            },
            IL::IsInt(_) => todo!(),
            IL::IsFloat(_) => todo!(),
            IL::IsUnary(_) => todo!(),
            IL::IsGround(_) => todo!(),
        }
    }
}

impl IL {
    pub fn new_link(link: &Link, mem_id: usize) -> Self {
        if let (Some(link1), Some(link2)) = (link.link1, link.link2) {
            IL::NewLink(link1.0.into(), link1.1, link2.0.into(), link2.1, mem_id)
        } else {
            panic!("Link is not fully defined")
        }
    }
}

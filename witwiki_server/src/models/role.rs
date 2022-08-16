use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Role {
    Admin,
    Maintainer,
    Reader,
}

// impl Hash for Role {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         std::mem::discriminant(&self).hash(state);
//     }
// }

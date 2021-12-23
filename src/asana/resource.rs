use fake::{Dummy, Fake};

/// Defines user data structure.
///
#[derive(Clone, Debug, Dummy, PartialEq)]
pub struct User {
    pub gid: String,
    pub name: String,
    pub email: String,
}

/// Defines workspace data structure.
///
#[derive(Clone, Debug, Dummy, PartialEq)]
pub struct Workspace {
    pub gid: String,
    pub name: String,
}

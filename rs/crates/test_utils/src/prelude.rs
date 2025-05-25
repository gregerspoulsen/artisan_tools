//! Convenient way to import all relevant types and traits from the crate
//! i.e. `use test_utils::prelude::*;`
pub use crate::{
    testremote::{RemoteRepo, RemoteRepoSetup, TestRemote},
    testrepo::{LocalRepo, TestRepo, WithRemote},
    GitRepo,
};

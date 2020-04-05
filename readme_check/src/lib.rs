//! This is a stub module that uses the unstable external_doc feature to run the
//! doc tests in the root README file. It is intended to be run in CI as a
//! backstop against breaking the intro hello world example.

#![feature(external_doc)]
#![doc(include = "../../README.md")]

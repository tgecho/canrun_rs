/*
The main distinction between "core" and "extra" goals is that core goals have
a custom run function while extras are made by combining other goals together.
This distinction is weakened a bit by extra goals that use lazy/custom. We'll
probably need to eventually revisit this categorization.
*/

pub mod all;
pub mod any;
pub mod append;
pub mod constrain;
pub mod map;
pub mod member;
pub mod relative;

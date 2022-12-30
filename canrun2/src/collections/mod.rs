/*! Unifiable collections with supporting [goal](crate::goals) functions.

# NOTE: These are not very battle tested and may be fatally flawed

Unifying large or complex collections may involve forking the state for
every possible combination of values. Also, the inherent complexity of
specifying and implementing these operations correctly means that they could
be flat out wrong. More testing, benchmarking and refinement is required.
*/

pub mod lvec;

use crate::{constrain, Can, CanT, Goal, LVar};

pub fn greater_than<'a, T: CanT + PartialOrd>(a: Can<T>, b: Can<T>) -> Goal<'a, T> {
    constrain(a, b, |a, b| a > b)
}
pub fn less_than<'a, T: CanT + PartialOrd>(a: Can<T>, b: Can<T>) -> Goal<'a, T> {
    constrain(a, b, |a, b| a < b)
}

pub trait RelativeComparison<'a, T: CanT + PartialOrd> {
    fn greater_than(self, other: Can<T>) -> Goal<'a, T>;
    fn less_than(self, other: Can<T>) -> Goal<'a, T>;
}
impl<'a, T: CanT + PartialOrd> RelativeComparison<'a, T> for Can<T> {
    fn greater_than(self, other: Can<T>) -> Goal<'a, T> {
        greater_than(self, other)
    }
    fn less_than(self, other: Can<T>) -> Goal<'a, T> {
        less_than(self, other)
    }
}
impl<'a, T: CanT + PartialOrd> RelativeComparison<'a, T> for LVar {
    fn greater_than(self, other: Can<T>) -> Goal<'a, T> {
        greater_than(self.can(), other)
    }
    fn less_than(self, other: Can<T>) -> Goal<'a, T> {
        less_than(self.can(), other)
    }
}

#[cfg(test)]
mod tests {
    use super::{greater_than, RelativeComparison};
    use crate::util::test;
    use crate::{val, Equals, LVar};

    #[test]
    fn relative_gt_1() {
        let x = LVar::labeled("x");
        let goals = vec![greater_than(x.can(), val(1)), x.equals(val(2))];
        let expected = vec![vec![val(2)]];
        test::all_permutations_resolve_to(goals, &vec![x], expected);
    }
    #[test]
    fn relative_gt_2() {
        let x = LVar::labeled("x");
        let goals = vec![x.greater_than(val(1)), x.equals(val(2))];
        let expected = vec![vec![val(2)]];
        test::all_permutations_resolve_to(goals, &vec![x], expected);
    }
    #[test]
    fn relative_gt_3() {
        let x = LVar::labeled("x");
        let goals = vec![greater_than(val(2), x.can()), x.equals(val(1))];
        let expected = vec![vec![val(1)]];
        test::all_permutations_resolve_to(goals, &vec![x], expected);
    }
    #[test]
    fn relative_gt_4() {
        let x = LVar::labeled("x");
        let goals = vec![x.equals(val(2)), val(1).less_than(x.can())];
        let expected = vec![vec![val(2)]];
        test::all_permutations_resolve_to(goals, &vec![x], expected);
    }
    #[test]
    fn relative_gt_5() {
        let x = LVar::labeled("x");
        let goals = vec![x.equals(val(1)), x.greater_than(val(2))];
        let expected = vec![];
        test::all_permutations_resolve_to(goals, &vec![x], expected);
    }
}

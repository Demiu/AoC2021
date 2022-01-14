use std::ops::RangeInclusive;

pub trait Intersect<Rhs: ?Sized = Self>: Sized {
    // associated type defaults are unstable
    // see issue #29661 <https://github.com/rust-lang/rust/issues/29661>
    type Output; // = Self;

    fn intersect_with(&self, other: &Rhs) -> Option<Self::Output>;
}

// Derived impl for slices of intersectable types
// Returns an Some of Vec of results of all intersections (including None's) or None if there aren't any
impl<T, U> Intersect<[U]> for T
where
    T: Intersect<U>,
{
    type Output = Vec<Option<T::Output>>;

    fn intersect_with(&self, other: &[U]) -> Option<Self::Output> {
        Some(other
            .iter()
            .map(|u| self.intersect_with(u))
            .collect::<Vec<_>>())
            .and_then(|v| if v.is_empty() { None } else { Some(v) })
    }
}

impl<Idx> Intersect for RangeInclusive<Idx>
where
    Idx: Ord + Copy,
{
    type Output = Self;

    fn intersect_with(&self, other: &Self) -> Option<Self::Output> {
        if self.start() > other.end() || other.start() > self.end() {
            None
        } else {
            let start = *self.start().max(other.start());
            let end = *self.end().min(other.end());
            Some(start..=end)
        }
    }
}

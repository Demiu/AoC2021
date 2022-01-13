use std::ops::RangeInclusive;

pub trait Intersect<Rhs: ?Sized = Self>: Sized {
    // associated type defaults are unstable
    // see issue #29661 <https://github.com/rust-lang/rust/issues/29661>
    type Output; // = Self;

    fn intersect_with(&self, other: &Rhs) -> Option<Self::Output>;
}

// Impl for slices
impl<T, U> Intersect<[U]> for T
where
    T: Intersect<U>,
{
    type Output = Vec<T::Output>;

    fn intersect_with(&self, other: &[U]) -> Option<Self::Output> {
        other
            .iter()
            .map(|u| self.intersect_with(u))
            .filter(Option::is_some)
            .collect::<Option<Vec<_>>>()
            .and_then(|v| if v.len() == 0 { None } else { Some(v) })
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

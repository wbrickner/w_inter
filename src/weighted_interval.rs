use std::ops::Add;
use crate::traits;

/// A batteries-included weighted interval representation.
/// 
/// NOTE: you can easily use your own representation (implement `Interval`, `Weighted`).
/// 
/// ```rust
/// // from tuple (start, end, weight)
/// let a = (1644526945376u64, 1644526960930u64, 137u8).into();
/// 
/// // a slightly more verbose equivalent
/// let b = WeightedInterval::new(-123i32, 123i32, 11u128);
/// ```
#[derive(Clone, Debug)]
pub struct WeightedInterval<Weight: Ord + Add + Clone, Time: Ord + Add + Clone> {
  pub start:  Time,
  pub end:    Time,
  pub weight: Weight
}

impl<Weight: Ord + Add + Clone + PartialEq, Time: Ord + Add + Clone + PartialEq> PartialEq for WeightedInterval<Weight, Time> {
  fn eq(&self, other: &Self) -> bool {
    self.start.eq(&other.start) &&
    self.end.eq(&other.end) &&
    self.weight.eq(&other.weight)
  }
}

impl<Weight: Ord + Add + Clone + PartialEq, Time: Ord + Add + Clone + PartialEq> Eq for WeightedInterval<Weight, Time> { }

impl<Weight: Ord + Add + Clone, Time: Ord + Add + Clone> WeightedInterval<Weight, Time> {
  pub fn new(
    start:  Time, 
    end:    Time, 
    weight: Weight
  ) -> Self { Self { start, end, weight } }
}

impl<Weight: Ord + Add + Clone, Time: Ord + Add + Clone> traits::Weighted<Weight> for WeightedInterval<Weight, Time> {
  fn weight(&self) -> Weight { self.weight.clone() }
}

impl<Weight: Ord + Add + Clone, Time: Ord + Add + Clone> traits::Interval<Time> for WeightedInterval<Weight, Time> {
  fn start(&self) -> Time { self.start.clone() }
  fn end(&self) -> Time { self.end.clone() }
}

impl<Weight: Ord + Add + Clone, Time: Ord + Add + Clone> From<(Time, Time, Weight)> for WeightedInterval<Weight, Time> {
  fn from(tuple: (Time, Time, Weight)) -> Self {
    let (start, end, weight) = tuple;
    Self { start, end, weight }
  }
}

use std::ops::Add;

/// If a type is `Interval`, it has bounds over a 1-dimensional domain.
pub trait Interval<Time: Ord> {
  fn start(&self) -> Time;
  fn end(&self) -> Time;
}

/// If a type is `Weighted`, it has some number-like value associated with it.
pub trait Weighted<Weight: Ord + Add> {
  fn weight(&self) -> Weight;
}
use std::ops::Add;
use crate::{traits, util::*};

/// Faster solver, only slightly more difficult to use correctly. `O(n log n)` in interval number.
/// 
/// This is the big-boy function, as such you must read and respect the rules:
/// 
/// - **`intervals` must be sorted ascending by interval end time.**
///   - If this invariant is not held, no claims are made about the behavior of this function.
///   - The output will be meaningless and internal integer overflows may occur.
/// 
/// - `memoization` is an existing buffer that will be written to (allowing us to amortize allocation costs).
///   - The critical requirement here is that `memoization.len() >= interval count`.
///   - There is no need to clear the contents between invocations.
/// 
/// - `solution` is an existing buffer that will be appended to (allowing us to amortize allocation costs).
///   - You can choose to clear the solution buffer when it contains old elements, or merge multiple solutions into the same buffer,
///     however you please.
/// 
/// # Example Usage
/// 
/// ```rust
/// // our goal is to allocate once and reuse the same buffers
/// // measure (or apply a guess) to avoid having to resize the vector.
/// let max_interval_count = problems.iter().map(|i| i.len()).max();
/// 
/// // we can say with certainty that the memo buffer 
/// // will never need to be larger than the largest input size.
/// let mut memo = vec![0u8; max_interval_count];
/// 
/// // we don't know how big the solution set will be, 
/// // but it can't be larger than the largest input size.
/// let mut soln = Vec::with_capacity(max_interval_count);
/// 
/// for intervals in problems {
///   // perhaps we know our intervals to be *almost* sorted, 
///   // so we choose to use an algorithm tuned for this case.
///   sort(&mut intervals);
///   
///   // we don't strictly need to, but we clear the old solution set,
///   // so that only the optimal set from this run ends up in the buffer.
///   soln.clear();
/// 
///   sorted(
///     &intervals,
///     &mut memo,
///     &mut soln
///   );
/// 
///   // we can now use the `soln` buffer before it's recycled
/// }
/// ```
/// 
pub fn sorted<Weight, Time, Interval, InputContainer>(
  intervals:   InputContainer,
  memoization: &mut [Weight],
  solution:    &mut Vec<Interval>
) where Weight: Ord + Add<Output = Weight> + Clone,
        Time: Ord,
        Interval: traits::Interval<Time> + traits::Weighted<Weight> + Clone,
        InputContainer: AsRef<[Interval]>
{
  let intervals = intervals.as_ref();
  
  if let Some(i) = intervals.get(0) { memoization[0] = i.weight(); }
  else { return; } // empty intervals

  // actually find the optimal solution
  internal(intervals, memoization, solution);
}

/// - `memoization` must have a first element, and already be of length `intervals.len()` or more.
/// - `optimal_solution` will be appended to. It should be empty if you want only the result of this computation.
fn internal<Weight, Time, Interval>(
  intervals:        &[Interval],
  memoization:      &mut [Weight],
  optimal_solution: &mut Vec<Interval>
) where Weight: Ord + Add<Output = Weight> + Clone,
        Time: Ord,
        Interval: traits::Interval<Time> + traits::Weighted<Weight> + Clone
{
  // build the memoization array
  for index in 1..intervals.len() {
    // find the last index compatible with the current interval
    let included_value = {
      let last = final_compatible(&intervals[..], index);

      if let Some(k) = last { intervals[index].weight() + memoization[k].clone() }
      else { intervals[index].weight() }
    };
    let excluded_value = memoization[index - 1].clone();
    memoization[index] = included_value.max(excluded_value);
  }

  // iteratively find the optimal solution
  let mut j = if intervals.len() != 0 { Some(intervals.len() - 1) } else { None };
  while let Some(i) = j {
    let last = final_compatible(&intervals[..], i);

    let z = {
      if let Some(k) = last { intervals[i].weight() + memoization[k].clone() }
      else { intervals[i].weight() }
    };
    
    if i == 0 || z > memoization[i - 1] {
      optimal_solution.push(intervals[i].clone());
      j = last;
    }
    else { j = Some(i - 1); }
  }
}

/// Marginally slower solver, impossible to misuse. `O(n log n)` in interval number.
/// - Should be pretty fast for most input.
/// - Overhead comes from sorting the input and allocating multiple times for each invocation of the solver.
/// - For better performance use `sorted`, which allows for fine-grain control and buffer reuse.
#[must_use]
pub fn unsorted<Weight, Time, Interval, InputContainer>(
  intervals: InputContainer
) -> Vec<Interval> 
  where Weight: Ord + Add<Output = Weight> + Clone,
        Time: Ord,
        Interval: traits::Interval<Time> + traits::Weighted<Weight> + Clone,
        InputContainer: AsRef<[Interval]> 
{
  // prepare an internal mutable clone, as the input is not known to be sorted
  let mut intervals = Vec::from(intervals.as_ref());

  // sort unstable by end time (unstable is *often* faster)
  intervals.sort_unstable_by(|a, b| a.end().cmp(&b.end()));

  // prepare memoization array (at most 1 alloc)
  let mut memoization = {
    let mut m: Vec<Weight> = Vec::with_capacity(intervals.len());
    if let Some(i) = intervals.get(0) { m.push(i.weight()); }
    m
  };

  // I have no guess as to the lenth of the optimal solution.
  let mut optimal_solution = vec![];

  // actually find the optimal solution
  internal(
    &intervals[..], 
    &mut memoization[..], 
    &mut optimal_solution
  );

  optimal_solution
}

#[cfg(test)]
mod tests {
  use crate::{WeightedInterval, unsorted};

  #[test]
  fn small_example() {
    let intervals = [
      WeightedInterval { start: 0u8, end: 6u8,  weight: 3u8 },
      WeightedInterval { start: 1u8, end: 4u8,  weight: 5u8 },
      WeightedInterval { start: 3u8, end: 5u8,  weight: 5u8 },
      WeightedInterval { start: 3u8, end: 8u8,  weight: 8u8 },
      WeightedInterval { start: 4u8, end: 7u8,  weight: 3u8 },
      WeightedInterval { start: 5u8, end: 9u8,  weight: 7u8 },
      WeightedInterval { start: 6u8, end: 10u8, weight: 3u8 },
      WeightedInterval { start: 8u8, end: 11u8, weight: 4u8 }
    ];

    let mut optimal_set = unsorted(&intervals);
    optimal_set.reverse();

    assert_eq!(optimal_set.len(), 2);

    assert_eq!(optimal_set[0].weight, intervals[1].weight);
    assert_eq!(optimal_set[0].start,  intervals[1].start);
    assert_eq!(optimal_set[0].end,    intervals[1].end);

    assert_eq!(optimal_set[1].weight, intervals[5].weight);
    assert_eq!(optimal_set[1].start,  intervals[5].start);
    assert_eq!(optimal_set[1].end,    intervals[5].end);
  }

  #[test]
  fn empty() {
    let intervals: [WeightedInterval<u8, u8>; 0] = [];

    let optimal_set = unsorted(&intervals);
    assert_eq!(optimal_set.len(), 0);
  }

  #[test]
  fn single() {
    let intervals = [
      WeightedInterval { start: 0, end: 128, weight: 15 }
    ];

    let optimal_set = unsorted(&intervals);
    assert_eq!(optimal_set.len(), 1);
    assert_eq!(optimal_set[0].start,  0);
    assert_eq!(optimal_set[0].end,    128);
    assert_eq!(optimal_set[0].weight, 15);
  }
}
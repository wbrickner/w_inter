use crate::traits::Interval;

/// - `s`: start time of `index`th interval
/// - `e`: end time of solution interval
/// 
/// Finds the index of the interval having maximum end time `e` such that `e <= s`.
pub fn final_compatible<Time: Ord, I: Interval<Time>>(intervals: &[I], index: usize) -> Option<usize> {
  if index == 0 { return None; }

  let mut low = 0;
  let mut high = index - 1;
  let target = intervals[index].start();

  let mut mid;
  while low < high {
    mid = low + (high - low + 1) / 2;
    if intervals[mid].end() <= target { low = mid; }
    else { high = mid - 1; }
  }
  if intervals[low].end() > target { return None; }
  
  return Some(low);
}
//! A pair of solvers for the [Weighted Interval Scheduling Problem](https://en.wikipedia.org/wiki/Interval_scheduling).
//! 
//! #### Features
//! - Zero external dependencies, although requires an allocator.
//! - Flexible: anything implementing `Ord + Add + Clone` may be thought of as an interval bound or a weight type.
//! - Efficient: running in `O(n log n)`.
//! - Fast: cache-aware, zero-reallocation APIs are available.
//! 
//! #### Simple Example
//! ```text
//!                    3───────────┐                    
//!                    └───────────┘                    
//!                8───────────────────┐                
//!                └───────────────────┘                
//!        5───────────┐   7───────────────┐            
//!        └───────────┘   └───────────────┘            
//!    3───────────────────────┐       4───────────┐    
//!    └───────────────────────┘       └───────────┘    
//!    2───┐       5───────┐   3───────────────┐        
//!    └───┘       └───────┘   └───────────────┘        
//! ◀──0───1───2───3───4───5───6───7───8───9───10──11──▶
//! ```
//! ```rust
//! let intervals = vec![
//!   (0u8, 1u8,  2u8).into(), // (start, end, weight)
//!   (0u8, 6u8,  3u8).into(),
//!   (1u8, 4u8,  5u8).into(),
//!   (3u8, 5u8,  5u8).into(),
//!   (3u8, 8u8,  8u8).into(),
//!   (4u8, 7u8,  3u8).into(),
//!   (5u8, 9u8,  7u8).into(),
//!   (6u8, 10u8, 3u8).into(),
//!   (8u8, 11u8, 4u8).into()
//! ];
//! 
//! let optimal = unsorted(&intervals);
//! 
//! assert_eq!(
//!   optimal, 
//!   vec![
//!     (1u8, 4u8, 5u8).into(),
//!     (5u8, 9u8, 7u8).into()
//!   ]
//! );
//! ```
//! 
//! #### Fast (Amortized Allocation) Example
//! <details>
//!   <summary>Click to view pseudocode</summary>
//!   
//! ```rust
//! // our goal is to allocate once and reuse the same buffers
//! // measure (or apply a guess) to avoid having to resize the vector.
//! let max_interval_count = problems.iter().map(|i| i.len()).max();
//! 
//! // we can say with certainty that the memo buffer 
//! // will never need to be larger than the largest input size.
//! let mut memo = vec![0u8; max_interval_count];
//! 
//! // we don't know how big the solution set will be, 
//! // but it can't be larger than the largest input size.
//! let mut soln = Vec::with_capacity(max_interval_count);
//! 
//! for intervals in problems {
//!   // perhaps we know our intervals to be *almost* sorted, 
//!   // so we choose to use an algorithm tuned for this case.
//!   sort(&mut intervals);
//!   
//!   // we don't strictly need to, but we clear the old solution set,
//!   // so that only the optimal set from this run ends up in the buffer.
//!   soln.clear();
//! 
//!   sorted(
//!     &intervals,
//!     &mut memo,
//!     &mut soln
//!   );
//! 
//!   // we can now use the `soln` buffer before it's recycled
//! }
//! ```
//! </details>
//! 
//! #### Outlook
//! 
//! I have ambitions to expand the scope to other (weighted) discrete optimization problems,
//! the ultimate goal being to provide an efficient, correct, and easy-to-use toolkit, documented in plain english, accessible to everyone.
//! 
//! I'm pretty busy. If you have an algorithm in mind, submit a pull request.
//! 
//! The public API will probably change as the project evolves.
//! 

mod traits;
mod util;
mod weighted_interval;
mod solvers;

pub use solvers::{sorted, unsorted};         // expose solver functions
pub use weighted_interval::WeightedInterval; // expose default weighted interval struct
pub use traits::{Interval, Weighted};        // expose traits so users can implement them on their own types
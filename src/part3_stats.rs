// Optional Follow-Up: Part 3
//
// Given a valid call tree, compute useful profiling statistics. This is
// intended as a follow-up only after Part 1 (and optionally Part 2) already
// feels educational.

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct CallSpan {
    name: String,
    start_ts: f64,
    end_ts: f64,
    children: Vec<CallSpan>,
}

#[derive(Debug, Clone, Default)]
struct Stats {
    total_time: f64,
    self_time: f64,
    call_count: usize,
}

fn compute_stats(_roots: &[CallSpan]) -> HashMap<String, Stats> {
    // Aggregate by function name:
    // - total_time
    // - self_time
    // - call_count
    //
    // Stubbed so you can practice the follow-up without mixing it into Part 1.
    HashMap::new()
}

fn main() {
    let roots = vec![CallSpan {
        name: "main".to_string(),
        start_ts: 1.0,
        end_ts: 5.0,
        children: vec![CallSpan {
            name: "work".to_string(),
            start_ts: 2.0,
            end_ts: 4.0,
            children: vec![],
        }],
    }];

    let stats = compute_stats(&roots);
    println!("{:#?}", stats);
}

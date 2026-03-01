// Interview Practice: Converting Stack Samples To A Trace
//
// Context
// -------
// Sampling profilers are a performance analysis tool for finding slow parts of
// a program by periodically sampling the current call stack.
//
// A lot of code may run between samples. That means you do not see every
// function call. You only see snapshots of what the stack looked like at a few
// points in time.
//
// In this problem, the input is a slice of `Sample`, already sorted by
// timestamp.
//
// Each sample contains:
// - `ts`: the sample timestamp
// - `stack`: the currently executing call stack
//
// The stack is ordered from outermost frame (like `main`) to the innermost
// frame.

#[derive(Debug, Clone)]
struct Sample {
    ts: f64,
    stack: Vec<String>,
}

// Task
// ----
// Convert those stack samples into a flat list of trace events that could be
// shown on a timeline.
//
// Each function call should produce:
// - a `"start"` event when it appears
// - an `"end"` event when it disappears
//
// Use the `Event` struct below for the output.

#[derive(Debug, Clone)]
struct Event {
    kind: String,
    ts: f64,
    name: String,
}

// Rules
// -----
// 1. Sample stacks contain every function that is currently executing.
// 2. Samples are in increasing timestamp order.
// 3. An unlimited amount of execution may happen between samples.
// 4. You do not need to reconstruct every real call that happened between
//    samples, only a valid nested trace that is consistent with the samples.
// 5. Assume call frames in the final sample have not finished yet, so do not
//    emit `"end"` events for frames still present in the last sample.
//
// The event order should be properly nested, as if it came from a real trace.
//
// For example:
//
//   |------outer------|
//        |--inner--|
//
// should be emitted in this order:
// - start outer
// - start inner
// - end inner
// - end outer
//
// Interview framing
// -----------------
// This is a good problem to talk through out loud:
// - restate the input/output shape
// - clarify edge cases
// - describe the invariant you want between consecutive samples
// - then implement incrementally
//
// It is fine to leave the function unimplemented at first and sketch examples.

fn convert_to_trace(_samples: &[Sample]) -> Vec<Event> {
    // Example
    // -------
    // For the sample input in `main()`, one valid output is:
    // Event { kind: "start", ts: 7.5, name: "main" }
    // Event { kind: "start", ts: 9.2, name: "my_fn" }
    // Event { kind: "start", ts: 9.2, name: "my_fn2" }
    // Event { kind: "end", ts: 10.7, name: "my_fn2" }
    // Event { kind: "end", ts: 10.7, name: "my_fn" }
    //
    // `main` is still present in the last sample, so there is no `"end"` event
    // for `main`.
    //
    // Stubbed so the project builds while you work on the function.
    Vec::new()
}

fn main() {
    let s1 = Sample {
        ts: 7.5,
        stack: vec!["main".to_string()],
    };
    let s2 = Sample {
        ts: 9.2,
        stack: vec![
            "main".to_string(),
            "my_fn".to_string(),
            "my_fn2".to_string(),
        ],
    };
    let s3 = Sample {
        ts: 10.7,
        stack: vec!["main".to_string()],
    };

    let samples = vec![s1, s2, s3];
    let events = convert_to_trace(&samples);

    for event in events {
        println!("{} {} {}", event.kind, event.ts, event.name);
    }
}

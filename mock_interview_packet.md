# Mock Interview Packet

## Trace Reconstruction From Sampled Call Stacks

This is a mock systems-style interview packet for practicing with a friend.

The problem is split into 3 parts. Each part builds on the previous one:

1. reconstruct a valid trace from sampled call stacks
2. turn that trace into explicit nested call spans
3. compute useful profiling statistics from those spans

The point is not to perfectly reconstruct a real execution. The point is to
produce one valid representation consistent with the observations and the rules.

This is not an official Anthropic interview prompt. It is a plausible mock
interview in a similar style: stateful, systems-adjacent, and grounded in
partial observations.

---

## Part 1: Convert Samples Into A Valid Trace

You are given periodic stack samples from a sampling profiler.

Each sample contains:
- a timestamp
- the currently executing call stack at that timestamp

The stack is ordered from outermost frame to innermost frame.

Use these types:

```rust
#[derive(Debug, Clone)]
struct Sample {
    ts: f64,
    stack: Vec<String>,
}

#[derive(Debug, Clone)]
struct Event {
    kind: String, // "start" or "end"
    ts: f64,
    name: String,
}
```

Implement:

```rust
fn convert_to_trace(samples: &[Sample]) -> Vec<Event>
```

The function should return a flat list of events representing a valid nested
execution trace.

### Behavioral Requirements

- A function should emit a `"start"` event when it begins.
- A function should emit an `"end"` event when it stops appearing in the sampled stack.
- Event order must be properly nested, as if it came from a real trace.
- If a function is nested inside another, its `"start"` must appear after the parent’s `"start"`, and its `"end"` must appear before the parent’s `"end"`.

Example nesting:

```text
|------outer------|
     |--inner--|
```

Corresponding event order:
- start outer
- start inner
- end inner
- end outer

### Important Assumptions

- Samples are already sorted by increasing timestamp.
- Sample stacks contain every function currently executing at that moment.
- An unlimited amount of execution may occur between samples.
- You do not need to reconstruct every real call that may have happened between samples.
- You only need to produce one valid nested trace consistent with the samples.
- Call frames still present in the final sample have not finished yet, so do not emit `"end"` events for them.

### Example

Input:

```rust
let samples = vec![
    Sample {
        ts: 7.5,
        stack: vec!["main".to_string()],
    },
    Sample {
        ts: 9.2,
        stack: vec![
            "main".to_string(),
            "my_fn".to_string(),
            "my_fn2".to_string(),
        ],
    },
    Sample {
        ts: 10.7,
        stack: vec!["main".to_string()],
    },
];
```

One valid output:

```rust
vec![
    Event { kind: "start".to_string(), ts: 7.5,  name: "main".to_string() },
    Event { kind: "start".to_string(), ts: 9.2,  name: "my_fn".to_string() },
    Event { kind: "start".to_string(), ts: 9.2,  name: "my_fn2".to_string() },
    Event { kind: "end".to_string(),   ts: 10.7, name: "my_fn2".to_string() },
    Event { kind: "end".to_string(),   ts: 10.7, name: "my_fn".to_string() },
]
```

Note: `main` is still present in the final sample, so there is no `"end"` event
for `main`.

### What This Part Tests

- modeling incomplete observations
- comparing adjacent states
- preserving nesting invariants
- avoiding over-reconstruction

---

## Part 2: Convert The Event Stream Into A Call Tree

You are now given a valid event stream like the one produced in Part 1.

The events are guaranteed to be:
- in chronological order
- properly nested
- balanced (every `"start"` has a matching `"end"`)

Use these types:

```rust
#[derive(Debug, Clone)]
struct Event {
    kind: String, // "start" or "end"
    ts: f64,
    name: String,
}

#[derive(Debug, Clone)]
struct CallSpan {
    name: String,
    start_ts: f64,
    end_ts: f64,
    children: Vec<CallSpan>,
}
```

Implement:

```rust
fn events_to_call_tree(events: &[Event]) -> Vec<CallSpan>
```

Return the list of root spans, in order.

### Behavioral Requirements

- A `"start"` event opens a new span.
- An `"end"` event closes the most recently opened still-active span.
- The `"end"` event's `name` should match the span it closes.
- If a span starts while another span is active, it becomes a child of that active span.
- If no span is active, the new span is a root span.
- Preserve sibling order from the input event stream.

### Assumptions

- The input event stream is valid and properly nested.
- Every `"start"` event has a matching `"end"` event.
- Timestamps are non-decreasing.
- Function names are not guaranteed to be globally unique.
- Nesting is determined by event order, not by matching names alone.

### Example

Input:

```rust
vec![
    Event { kind: "start".to_string(), ts: 1.0, name: "main".to_string() },
    Event { kind: "start".to_string(), ts: 2.0, name: "foo".to_string() },
    Event { kind: "end".to_string(),   ts: 3.0, name: "foo".to_string() },
    Event { kind: "start".to_string(), ts: 4.0, name: "bar".to_string() },
    Event { kind: "end".to_string(),   ts: 5.0, name: "bar".to_string() },
    Event { kind: "end".to_string(),   ts: 6.0, name: "main".to_string() },
]
```

Output:

```rust
vec![
    CallSpan {
        name: "main".to_string(),
        start_ts: 1.0,
        end_ts: 6.0,
        children: vec![
            CallSpan {
                name: "foo".to_string(),
                start_ts: 2.0,
                end_ts: 3.0,
                children: vec![],
            },
            CallSpan {
                name: "bar".to_string(),
                start_ts: 4.0,
                end_ts: 5.0,
                children: vec![],
            },
        ],
    },
]
```

### What This Part Tests

- parsing nested structure from a flat representation
- stack-based reasoning
- building explicit hierarchical data

---

## Part 3: Compute Profiling Statistics

You are now given a valid call tree like the one produced in Part 2.

Compute aggregated statistics by function name.

Use these types:

```rust
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
```

Implement:

```rust
use std::collections::HashMap;

fn compute_stats(roots: &[CallSpan]) -> HashMap<String, Stats>
```

Return a map from function name to aggregated stats across all spans.

### Definitions

- The duration of a span is `end_ts - start_ts`.
- `total_time` is the sum of durations of all spans with that function name.
- The `self_time` of a span is its duration minus the sum of the durations of its direct child spans.
- The `self_time` for a function is the sum of self times across all spans with that name.
- `call_count` is the number of spans with that function name.

### Assumptions

- The input tree is valid.
- Every child span is fully contained within its parent.
- Sibling spans do not overlap.
- Basic `f64` arithmetic is acceptable for timestamps.

### Example

Input:

```rust
vec![
    CallSpan {
        name: "main".to_string(),
        start_ts: 1.0,
        end_ts: 10.0,
        children: vec![
            CallSpan {
                name: "foo".to_string(),
                start_ts: 2.0,
                end_ts: 5.0,
                children: vec![],
            },
            CallSpan {
                name: "bar".to_string(),
                start_ts: 6.0,
                end_ts: 9.0,
                children: vec![],
            },
        ],
    },
]
```

Output conceptually:

```rust
{
    "main" => Stats {
        total_time: 9.0,
        self_time: 3.0,
        call_count: 1,
    },
    "foo" => Stats {
        total_time: 3.0,
        self_time: 3.0,
        call_count: 1,
    },
    "bar" => Stats {
        total_time: 3.0,
        self_time: 3.0,
        call_count: 1,
    },
}
```

### What This Part Tests

- converting structure into useful analysis
- distinguishing inclusive vs exclusive work
- tree traversal
- aggregation

---

## Suggested Interview Flow

1. Present only Part 1 at first.
2. Let the candidate ask clarifying questions.
3. Encourage them to restate the problem before coding.
4. If they finish or make strong progress, reveal Part 2.
5. If they finish or make strong progress again, reveal Part 3.
6. End with a short debrief on tradeoffs, edge cases, and testing.

This format works better than dumping all 3 parts at once.

---

## Suggested Interviewer Prompts

- What assumptions are you making?
- What makes an output valid here?
- What invariant are you maintaining between adjacent samples?
- Can you walk through your algorithm on the example?
- What edge cases are you thinking about?
- How would you test this?
- Can you make the intermediate representation more explicit?

---

## Allowed Hints

- Part 1: Try comparing consecutive samples.
- Part 1: What stays the same between two adjacent stacks?
- Part 2: You already have properly nested events. What data structure fits that?
- Part 3: What is the difference between total time and time spent only in this frame?

---

## Candidate Guidance

The candidate should:

- ask clarifying questions early
- state assumptions explicitly
- define a small invariant before coding
- solve incrementally
- use examples to validate logic
- avoid overfitting to real profiler internals beyond the written rules

This is not a trick question. The difficulty comes from modeling partial
observations cleanly and preserving structure as you move between layers.

---

## Difficulty Calibration

- Part 1: medium to medium-hard in interview conditions
- Part 2: easier than Part 1 if the candidate recognizes the nesting structure
- Part 3: medium, with more emphasis on definitions and traversal

Taken together, this is a strong mid-level to senior systems-flavored mock
interview.

---

## Fun Mode Variant

To keep it playful:

- Timebox each part separately.
- Have the interviewer stay in character.
- Allow one deliberate ambiguity in Part 1 and see if the candidate catches it.
- End with one strong-hire signal.
- End with one not-yet signal.
- End with one thing to practice next.

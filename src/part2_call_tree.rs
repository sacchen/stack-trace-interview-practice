// Optional Follow-Up: Part 2
//
// Given a valid, properly nested event stream, build an explicit tree of call
// spans. This is intended as a follow-up only after Part 1 is already useful to
// you.

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

fn events_to_call_tree(_events: &[Event]) -> Vec<CallSpan> {
    // Build the list of root spans, in order.
    //
    // Assumptions:
    // - the event stream is valid
    // - events are properly nested
    // - every "start" has a matching "end"
    //
    // Stubbed so you can practice the follow-up without mixing it into Part 1.
    Vec::new()
}

fn main() {
    let events = vec![
        Event {
            kind: "start".to_string(),
            ts: 1.0,
            name: "main".to_string(),
        },
        Event {
            kind: "start".to_string(),
            ts: 2.0,
            name: "foo".to_string(),
        },
        Event {
            kind: "end".to_string(),
            ts: 3.0,
            name: "foo".to_string(),
        },
        Event {
            kind: "end".to_string(),
            ts: 4.0,
            name: "main".to_string(),
        },
    ];

    let roots = events_to_call_tree(&events);
    println!("{:#?}", roots);
}

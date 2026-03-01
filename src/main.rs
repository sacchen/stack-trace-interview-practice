// Problem: Converting stack samples to a trace
//
// Sampling profilers are a performance analysis tool for finding the slow parts
// of your code by periodically sampling the entire call stack (lots of code
// might run between samples). In our problem the samples will be a Vec<Sample>
// of a float timestamp and a vector of function names, in order by timestamp.

#[derive(Debug, Clone)]
struct Sample {
    ts: f64,
    stack: Vec<String>,
}

// Sometimes it's nice to visualize these samples on a chronological timeline of
// the call stack using a trace visualizer UI. To do this we need to convert the
// samples into a list of start and end events for each function call.
//
// Assume call frames in the last sample have not finished.

#[derive(Debug, Clone)]
struct Event {
    kind: String,
    ts: f64,
    name: String,
}

fn convert_to_trace(_samples: &[Sample]) -> Vec<Event> {
    // Expected output for the example in main():
    // Event { kind: "start", ts: 7.5, name: "main" }
    // Event { kind: "start", ts: 9.2, name: "my_fn" }
    // Event { kind: "end", ts: 10.7, name: "my_fn" }
    //
    // Stubbed so the project builds and runs while you work on the function.
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

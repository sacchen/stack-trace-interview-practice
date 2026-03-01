#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

// Problem: Converting stack samples to a trace

// Sampling profilers are a performance analysis tool for finding the slow parts
// of your code by periodically sampling the entire call stack (lots of code
// might run between samples). In our problem the samples will be a vector of
// 'Sample's of a float timestamp and a vector of function names, *in order by
// timestamp*, like this:

struct Sample {
    double ts;
    std::vector<std::string> stack;
};

// Sometimes it's nice to visualize these samples on a chronological timeline of
// the call stack using a trace visualizer UI. To do this we need to *convert*
// the samples into a list of start and end events for each function call.*

// Sample stacks contain every function that is currently executing.
// The stacks are in order from outermost function (like "main") to the
// innermost currently executing function. An unlimited amount of
// execution/change can happen between samples. We don't have all the function
// calls that happened, just some samples we want to visualize.

// The returned events should be in a list order properly nested as if
// they were recorded. For example, a nested function call's start event is
// after the one from the enclosing function, and the nested call's end event is
// before the enclosing call's end event.

//   |------outer------|
//        |--inner--|

// would be ordered: start outer, start inner, end inner, end outer

// *Assume call frames in the last sample haven't finished.* The resulting
// events should use the Event struct:

struct Event {
    std::string kind;
    double ts;
    std::string name;

    void PrintDebug() const {
        std::cout << "Kind : " << kind << std::endl;
        std::cout << "ts : " << ts << std::endl;
        std::cout << "Name : " << name << std::endl;
        std::cout << "----" << std::endl;
    }
};

std::vector<Event> convertToTrace(const std::vector<Sample>& samples) {
    // Expected output for the example in main():
    // Event{"start", 7.5, "main"}
    // Event{"start", 9.2, "my_fn"}
    // Event{"end", 10.7, "my_fn"}

    // Stubbed so the project builds and runs while you work on the function.
    return {};
}

int main() {
    Sample s1{7.5, {"main"}};
    Sample s2{9.2, {"main", "my_fn", "my_fn2"}};
    Sample s3{10.7, {"main"}};

    std::vector<Sample> samples = {s1, s2, s3};

    auto events = convertToTrace(samples);

    for (const auto& e : events) {
        std::cout << e.kind << " " << e.ts << " " << e.name << "\n";
    }
}

#pragma once

#include <chrono>
#include <iostream>
#include <string>

class ScopedTimer {
public:
    ScopedTimer(std::string name)
        : name_(std::move(name)),
          start_(std::chrono::high_resolution_clock::now()) {}

    ~ScopedTimer() {
        auto end = std::chrono::high_resolution_clock::now();
        auto duration =
            std::chrono::duration_cast<std::chrono::microseconds>(end - start_);

        std::cout << name_ << " took "
                  << duration.count() << "us\n";
    }

private:
    std::string name_;
    std::chrono::high_resolution_clock::time_point start_;
};

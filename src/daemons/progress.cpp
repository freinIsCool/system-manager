#include "ui/ui.hpp"

#include <chrono>
#include <cmath>
#include <thread>

void run_progress()
{
    double i = 0.0;

    while (i < 1.0)
    {
        fraction += 0.1;

        i += 0.1;

        std::this_thread::sleep_for(
            std::chrono::milliseconds(100)
        );
    }

    if (std::abs(fraction - 1.0) < 1e-9)
    {
        fraction -= 1.0;
    }
}
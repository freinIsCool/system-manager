#include "daemons/progress.hpp"
#include "ui/ui.hpp"
#include <thread>


int main(int argc, char* argv[])
{
    return run_app(argc, argv);

    std::thread progress_thread(run_progress);
    progress_thread.join();
}

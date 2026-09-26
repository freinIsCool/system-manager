#pragma once

#include <gtkmm.h>

class UI
{
public:
    UI();

    void start_progress();

private:
    void update_progress();

    Gtk::ProgressBar progress_bar;
    Glib::Dispatcher progress_dispatcher;

    double fraction = 0.0;
};
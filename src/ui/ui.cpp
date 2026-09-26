#include <gtkmm.h>
#include "ui.hpp"

class MyWindow : public Gtk::Window
{
public:
    MyWindow()
    {
        float fraction = 0.0;
        set_title("system manager");
        set_default_size(250, 250);
        
        m_button.set_label("bar");

        m_button.set_halign(Gtk::Align::END);
        m_button.set_valign(Gtk::Align::CENTER);

        m_ProgressBar.set_halign(Gtk::Align::CENTER);
        m_ProgressBar.set_valign(Gtk::Align::CENTER); // Fixed typo here (was set_halign twice)
        
        m_ProgressBar.set_size_request(100, -1); // Width: 100px, Height: -1 (default)   
        m_ProgressBar.set_fraction(fraction);

        // 1. Attach widgets to the grid container
        m_grid.attach(m_button, 0, 0, 1, 1);
        m_grid.attach(m_ProgressBar, 2, 0, 2, 1);

        // 2. Set the grid as the main child widget of the window
        set_child(m_grid);
    }

private:
    Gtk::ProgressBar m_ProgressBar;
    Gtk::Button m_button;
    Gtk::Grid m_grid;
};

int run_app(int argc, char* argv[])
{
    auto app = Gtk::Application::create("org.frein.systemmanager");
    return app->make_window_and_run<MyWindow>(argc, argv);
}

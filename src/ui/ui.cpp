#include <gtkmm.h>
#include "ui.hpp"

class MyWindow : public Gtk::Window
{
public:
    MyWindow()
    {
        set_title("Basic application");
        set_default_size(200, 200);
        set_child(m_button);
    }

private:
    Gtk::Button m_button{"Hello"};
};

int run_app(int argc, char* argv[])
{
    auto app = Gtk::Application::create("org.frein.systemmanager");
    return app->make_window_and_run<MyWindow>(argc, argv);
}   
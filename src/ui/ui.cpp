#include <gtkmm.h>
#include "ui.hpp"

class MyWindow : public Gtk::Window
{
public:
    MyWindow()
    {
        set_title("Basic application");
        set_default_size(250, 250);
        
        m_label.set_text("This is simple, static text.");

        m_label.set_halign(Gtk::Align::START);
        m_label.set_valign(Gtk::Align::CENTER);

        set_child(m_label);
    }

private:
  Gtk::Label m_label;
};

int run_app(int argc, char* argv[])
{
    auto app = Gtk::Application::create("org.frein.systemmanager");
    return app->make_window_and_run<MyWindow>(argc, argv);
}   
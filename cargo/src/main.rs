mod bar_update;
mod widgets;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, CssProvider, glib};
use gtk4::gdk::Display;

use crate::{bar_update::schedule_bar_updates, widgets::build_widgets};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("Org.Frein.SystemManager")
        .build();

    app.connect_activate(|app| {
        let css_provider = CssProvider::new();

        // load css

        let css_file = gtk4::gio::File::for_path("style.css");
        css_provider.load_from_file(&css_file);

        if let Some(display) = Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &css_provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        let widgets = build_widgets();
        widgets.box_.add_css_class("system-panel");

        let window = ApplicationWindow::builder()
            .child(&widgets.box_)
            .application(app)
            .title("System Manager")
            .build();

        window.set_decorated(true);

        let toggle_button = widgets.toggle_titlebar_button.clone();
        let close_button = widgets.close_button.clone();
        let hidden_window = window.clone();
        let panel = widgets.box_.clone();

        let toggle_button_for_click = toggle_button.clone();
        let close_button_for_toggle = close_button.clone();
        let hidden_window_for_toggle = hidden_window.clone();
        let panel_for_toggle = panel.clone();

        toggle_button_for_click.set_label("Hide Titlebar/background");
        close_button_for_toggle.set_visible(false);

        toggle_button_for_click.clone().connect_clicked(move |_| {
            let is_decorated = hidden_window_for_toggle.is_decorated();
            hidden_window_for_toggle.set_decorated(!is_decorated);

            if is_decorated {
                panel_for_toggle.add_css_class("background-hidden");
                hidden_window_for_toggle.add_css_class("transparent-window");
                close_button_for_toggle.set_visible(true);
                toggle_button_for_click.set_label("Show Titlebar/background");
            } else {
                panel_for_toggle.remove_css_class("background-hidden");
                hidden_window_for_toggle.remove_css_class("transparent-window");
                close_button_for_toggle.set_visible(false);
                toggle_button_for_click.set_label("Hide Titlebar/background");
            }
        });

        let hidden_window_for_close = hidden_window.clone();
        close_button.connect_clicked(move |_| {
            hidden_window_for_close.destroy();
        });

        window.present();

        schedule_bar_updates(
            widgets.memory_bar,
            widgets.storage_bar,
            widgets.cpu_bar,
            widgets.swap_bar,
            widgets.vram_bar,
            widgets.system_load_bar,
            widgets.memory_usage_fraction,
            widgets.storage_usage_fraction,
            widgets.cpu_usage_fraction,
            widgets.swap_usage_fraction,
            widgets.vram_usage_fraction,
            widgets.system_load_fraction,
            widgets.memory_usage_label,
            widgets.storage_usage_label,
            widgets.cpu_usage_label,
            widgets.swap_usage_label,
            widgets.vram_usage_label,
            widgets.system_load_usage_label,
        );
    });

    app.run()
}
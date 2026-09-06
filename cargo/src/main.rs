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
        css_provider.load_from_data(
            "window { background-color: #20293a; }\n\
             window.transparent-window { background-color: transparent; }\n\
             #system-panel { background-color: rgba(32, 41, 58, 0.96); border-radius: 12px; padding: 0; }\n\
             .title-label { font-size: 18px; font-weight: 700; color: #edf5ff; }\n\
             .metric-label { font-size: 14px; color: #dfe9f9; text-shadow: 2px 0 #000000, -1px 0 #111827, 0 1px #111827, 0 -1px #111827; }\n\
             .value-label { font-size: 13px; color: #dfe9f9; font-weight: 700; text-shadow: 2px 0 #000000, -1px 0 #111827, 0 1px #111827, 0 -1px #111827; }\n\
             .title-button { font-size: 12px; color: #edf5ff; background-color: rgba(255, 255, 255, 0.04); border: 1px solid rgba(163, 170, 255, 0.75); border-radius: 8px; padding: 6px 12px; }\n\
             .close-button { border-color: rgba(172, 138, 255, 0.0); }\n\
             .system-panel.background-hidden { background-color: transparent; }\n\
             .meter { min-width: 190px; }\n\
             progressbar { min-height: 12px; }\n\
             progressbar trough { min-height: 12px; border-radius: 3px; background: rgba(138, 150, 180, 0.35); }\n\
             progressbar progress { min-height: 12px; border-radius: 3px; background: #8ab7ff; }\n",
        );

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
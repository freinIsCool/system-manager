use gtk4::prelude::*;
use gtk4::{Box, Button, Label, ProgressBar};
use std::cell::Cell;
use std::rc::Rc;

pub struct Widgets {
    pub box_: Box,
    pub memory_bar: ProgressBar,
    pub storage_bar: ProgressBar,
    pub cpu_bar: ProgressBar,
    pub swap_bar: ProgressBar,
    pub vram_bar: ProgressBar,
    pub system_load_bar: ProgressBar,
    pub memory_usage_fraction: Rc<Cell<f64>>,
    pub storage_usage_fraction: Rc<Cell<f64>>,
    pub cpu_usage_fraction: Rc<Cell<f64>>,
    pub swap_usage_fraction: Rc<Cell<f64>>,
    pub vram_usage_fraction: Rc<Cell<f64>>,
    pub system_load_fraction: Rc<Cell<f64>>,
    pub memory_usage_label: Label,
    pub storage_usage_label: Label,
    pub cpu_usage_label: Label,
    pub swap_usage_label: Label,
    pub vram_usage_label: Label,
    pub system_load_usage_label: Label,
    pub toggle_titlebar_button: Button,
    pub close_button: Button,
}

fn create_resource_row(label_text: &str, bar: &ProgressBar, usage_label: &Label) -> Box {
    let row = Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .spacing(12)
        .margin_top(0)
        .margin_bottom(0)
        .margin_start(12)
        .margin_end(12)
        .build();

    let label = Label::new(Some(label_text));
    label.set_width_chars(16);
    label.set_xalign(0.0);
    label.add_css_class("metric-label");

    let value = usage_label.clone();
    value.set_width_chars(13);
    value.set_xalign(0.0);
    value.add_css_class("value-label");

    let bar_box = Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .width_request(190)
        .build();
    bar_box.append(bar);

    row.append(&label);
    row.append(&value);
    row.append(&bar_box);
    row
}

pub fn build_widgets() -> Widgets {
    let box_ = Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(4)
        .build();

    let title_row = Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .margin_top(6)
        .margin_start(12)
        .margin_end(12)
        .margin_bottom(4)
        .build();

    let title = Label::new(Some("System Manager"));
    title.add_css_class("title");

    let spacer = Box::builder().hexpand(true).build();
    let toggle_titlebar_button = Button::with_label("Hide Titlebar");
    let close_button = Button::with_label("Close");

    title_row.append(&title);
    title_row.append(&spacer);
    title_row.append(&toggle_titlebar_button);
    title_row.append(&close_button);
    box_.append(&title_row);

    let memory_bar = ProgressBar::builder().show_text(false).build();
    memory_bar.set_fraction(0.0);
    memory_bar.add_css_class("meter");

    let storage_bar = ProgressBar::builder().show_text(false).build();
    storage_bar.set_fraction(0.0);
    storage_bar.add_css_class("meter");

    let cpu_bar = ProgressBar::builder().show_text(false).build();
    cpu_bar.set_fraction(0.0);
    cpu_bar.add_css_class("meter");

    let swap_bar = ProgressBar::builder().show_text(false).build();
    swap_bar.set_fraction(0.0);
    swap_bar.add_css_class("meter");

    let vram_bar = ProgressBar::builder().show_text(false).build();
    vram_bar.set_fraction(0.0);
    vram_bar.add_css_class("meter");

    let system_load_bar = ProgressBar::builder().show_text(false).build();
    system_load_bar.set_fraction(0.0);
    system_load_bar.add_css_class("meter");

    let memory_usage_label = Label::new(Some("0%"));
    let storage_usage_label = Label::new(Some("0%"));
    let cpu_usage_label = Label::new(Some("0%"));
    let swap_usage_label = Label::new(Some("0%"));
    let vram_usage_label = Label::new(Some("0%"));
    let system_load_usage_label = Label::new(Some("0%"));

    let memory_row = create_resource_row("Memory Usage", &memory_bar, &memory_usage_label);
    let storage_row = create_resource_row("Storage Usage", &storage_bar, &storage_usage_label);
    let cpu_row = create_resource_row("CPU Usage", &cpu_bar, &cpu_usage_label);
    let swap_row = create_resource_row("Swap Usage", &swap_bar, &swap_usage_label);
    let vram_row = create_resource_row("Virtual Memory Usage", &vram_bar, &vram_usage_label);
    let load_row = create_resource_row("System Load", &system_load_bar, &system_load_usage_label);

    box_.append(&memory_row);
    box_.append(&storage_row);
    box_.append(&cpu_row);
    box_.append(&swap_row);
    box_.append(&vram_row);
    box_.append(&load_row);

    Widgets {
        box_,
        memory_bar,
        storage_bar,
        cpu_bar,
        swap_bar,
        vram_bar,
        system_load_bar,
        memory_usage_fraction: Rc::new(Cell::new(0.0)),
        storage_usage_fraction: Rc::new(Cell::new(0.0)),
        cpu_usage_fraction: Rc::new(Cell::new(0.0)),
        swap_usage_fraction: Rc::new(Cell::new(0.0)),
        vram_usage_fraction: Rc::new(Cell::new(0.0)),
        system_load_fraction: Rc::new(Cell::new(0.0)),
        memory_usage_label,
        storage_usage_label,
        cpu_usage_label,
        swap_usage_label,
        vram_usage_label,
        system_load_usage_label,
        toggle_titlebar_button,
        close_button,
    }
}

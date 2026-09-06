use gtk4::{glib, Label, ProgressBar};
use std::cell::Cell;
use std::path::Path;
use std::rc::Rc;
use sysinfo::System;

pub fn schedule_bar_updates(
    memory_bar: ProgressBar,
    storage_bar: ProgressBar,
    cpu_bar: ProgressBar,
    swap_bar: ProgressBar,
    vram_bar: ProgressBar,
    system_load_bar: ProgressBar,
    memory_usage_fraction: Rc<Cell<f64>>,
    storage_usage_fraction: Rc<Cell<f64>>,
    cpu_usage_fraction: Rc<Cell<f64>>,
    swap_usage_fraction: Rc<Cell<f64>>,
    vram_usage_fraction: Rc<Cell<f64>>,
    system_load_fraction: Rc<Cell<f64>>,
    memory_usage_label: Label,
    storage_usage_label: Label,
    cpu_usage_label: Label,
    swap_usage_label: Label,
    vram_usage_label: Label,
    system_load_usage_label: Label,
) {
    let mut sys = System::new_all();

    glib::timeout_add_seconds_local(1, move || {
        sys.refresh_all();

        let memory_fraction = if sys.total_memory() > 0 {
            sys.used_memory() as f64 / sys.total_memory() as f64
        } else {
            0.0
        };
        memory_usage_fraction.set(memory_fraction);
        memory_bar.set_fraction(memory_fraction);

        let disks = sysinfo::Disks::new_with_refreshed_list();
        let (total_storage, used_storage) = disks
            .iter()
            .find(|disk| disk.mount_point() == Path::new("/"))
            .map(|disk| {
                (
                    disk.total_space(),
                    disk.total_space().saturating_sub(disk.available_space()),
                )
            })
            .unwrap_or((0, 0));
        let total_storage = total_storage as f64;
        let used_storage = used_storage as f64;
        let storage_fraction = if total_storage > 0.0 {
            used_storage / total_storage
        } else {
            0.0
        };
        storage_usage_fraction.set(storage_fraction);
        storage_bar.set_fraction(storage_fraction);

        let used_storage_gb = used_storage / (1024.0 * 1024.0 * 1024.0);
        let total_storage_gb = total_storage / (1024.0 * 1024.0 * 1024.0);
        storage_usage_label.set_text(&format!(
            "{:.1} GB / {:.1} GB",
            used_storage_gb, total_storage_gb
        ));

        let used_memory_gb = sys.used_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let total_memory_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        memory_usage_label.set_text(&format!(
            "{:.1} GB / {:.1} GB",
            used_memory_gb, total_memory_gb
        ));

        let cpu_usage = sys.global_cpu_usage();
        let cpu_fraction = (cpu_usage / 100.0).clamp(0.0, 1.0) as f64;
        cpu_usage_fraction.set(cpu_fraction);
        cpu_bar.set_fraction(cpu_fraction);
        cpu_usage_label.set_text(&format!("{:.0}%", cpu_usage));

        let swap_fraction = if sys.total_swap() > 0 {
            sys.used_swap() as f64 / sys.total_swap() as f64
        } else {
            0.0
        };
        swap_usage_fraction.set(swap_fraction);
        swap_bar.set_fraction(swap_fraction);

        let used_swap_gb = sys.used_swap() as f64 / (1024.0 * 1024.0 * 1024.0);
        let total_swap_gb = sys.total_swap() as f64 / (1024.0 * 1024.0 * 1024.0);
        swap_usage_label.set_text(&format!("{:.1} GB / {:.1} GB", used_swap_gb, total_swap_gb));

        let vram_fraction = if sys.total_memory() > 0 {
            sys.used_memory() as f64 / sys.total_memory() as f64
        } else {
            0.0
        };
        vram_usage_fraction.set(vram_fraction);
        vram_bar.set_fraction(vram_fraction);
        let used_vram_gb = sys.used_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let total_vram_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        vram_usage_label.set_text(&format!("{:.1} GB / {:.1} GB", used_vram_gb, total_vram_gb));

        let load_avg = System::load_average();
        let core_count = std::thread::available_parallelism()
            .map(|n| n.get() as f64)
            .unwrap_or(1.0);
        let load_fraction = (load_avg.one / core_count).min(1.0).max(0.0);
        system_load_fraction.set(load_fraction);
        system_load_bar.set_fraction(load_fraction);
        system_load_usage_label.set_text(&format!("{:.2} / {:.0} cores", load_avg.one, core_count));

        glib::ControlFlow::Continue
    });
}

import tkinter as tk
from tkinter import ttk
import customtkinter as ctk
import psutil

ctk.set_default_color_theme("blue")
ctk.set_appearance_mode("system")

def update_stats():
    vm = psutil.virtual_memory()
    cpu_percent = psutil.cpu_percent(interval=None)
    disk = psutil.disk_usage('/')
    swap = psutil.swap_memory()
    vmem = psutil.virtual_memory()
    sys_load = psutil.getloadavg()
    num_cpus = psutil.cpu_count()

    load_percentages = [(load / num_cpus) * 100 for load in sys_load]

    cpu_bar.set(cpu_percent / 100)
    progress_bar.set(vm.percent / 100)
    storage_bar.set(disk.percent / 100)
    swap_bar.set(swap.percent / 100)
    virtual_memory_bar.set(vmem.percent / 100)
    sys_load_bar.set(load_percentages[0] / 100)

    root.after(1000, update_stats)
    MemPercentLabel.configure(text=f"{vm.percent:.1f}%")
    StoragePercentLabel.configure(text=f"{disk.percent:.1f}%")
    CPUPercentLabel.configure(text=f"{cpu_percent:.1f}%")
    SwapPercentLabel.configure(text=f"{swap.percent:.1f}%")
    VirtualMemoryPercentLabel.configure(text=f"{vmem.percent:.1f}%")
    sys_load_percent_label.configure(text=f"{load_percentages[0]:.2f}%")

    return (
        f"CPU: {cpu_percent}%, Memory: {vm.percent}%, Storage: {disk.percent}%, Swap: {swap.percent}%, Virtual Memory: {vmem.percent}%, System Load: {load_percentages[0]:.2f}%")

root = ctk.CTk()
root.title("System Manager")
root.geometry("355x235")

progStyle = ctk.CTkProgressBar(root, orientation="horizontal", width=300, height=50, corner_radius=10, border_width=2, border_color="#BDBDBD", progress_color="#4CAF50", fg_color="#E0E0E0")

frm = ctk.CTkFrame(master=root, fg_color="transparent") 
frm.pack(fill="both", expand=True)

MemLabel = ctk.CTkLabel(frm, text="Memory Usage", anchor="w", text_color="#cad3f5")
StorageLabel = ctk.CTkLabel(frm, text="Storage Usage", anchor="w", text_color="#cad3f5")
cpuLabel = ctk.CTkLabel(frm, text="CPU Usage", anchor="w", text_color="#cad3f5")

progress_bar = ctk.CTkProgressBar(frm, orientation="horizontal", width=120, height=20, corner_radius=9, fg_color="#363a4f", progress_color="#8aadf4")
storage_bar = ctk.CTkProgressBar(frm, orientation="horizontal", width=120, height=20, corner_radius=9, fg_color="#363a4f", progress_color="#8aadf4")
cpu_bar = ctk.CTkProgressBar(frm, orientation="horizontal", width=120, height=20, corner_radius=9, fg_color="#363a4f", progress_color="#8aadf4")
swap_bar = ctk.CTkProgressBar(frm, orientation="horizontal", width=120, height=20, corner_radius=9, fg_color="#363a4f", progress_color="#8aadf4")
virtual_memory_bar = ctk.CTkProgressBar(frm, orientation="horizontal", width=120, height=20, corner_radius=9, fg_color="#363a4f", progress_color="#8aadf4")
sys_load_bar = ctk.CTkProgressBar(frm, orientation="horizontal", width=120, height=20, corner_radius=9, fg_color="#363a4f", progress_color="#8aadf4")

MemPercentLabel = ctk.CTkLabel(frm, text="0%", anchor="w", text_color="#cad3f5")
StoragePercentLabel = ctk.CTkLabel(frm, text="0%", anchor="w", text_color="#cad3f5")
CPUPercentLabel = ctk.CTkLabel(frm, text="0%", anchor="w", text_color="#cad3f5")
SwapPercentLabel = ctk.CTkLabel(frm, text="0%", anchor="w", text_color="#cad3f5")
VirtualMemoryPercentLabel = ctk.CTkLabel(frm, text="0%", anchor="w", text_color="#cad3f5")
sys_load_percent_label = ctk.CTkLabel(frm, text="0%", anchor="w", text_color="#cad3f5")

MemLabel.grid(row=0, column=0, padx=5, pady=5, sticky="w")
progress_bar.grid(row=0, column=1, padx=5, pady=5)
MemPercentLabel.grid(row=0, column=2, padx=5, pady=5, sticky="e")
sys_load_percent_label.grid(row=5, column=2, padx=5, pady=5, sticky="e")

StorageLabel.grid(row=1, column=0, padx=5, pady=5, sticky="w")
storage_bar.grid(row=1, column=1, padx=5, pady=5)
StoragePercentLabel.grid(row=1, column=2, padx=5, pady=5, sticky="e")
sys_load_label = ctk.CTkLabel(frm, text="System Load", anchor="w", text_color="#cad3f5")
sys_load_label.grid(row=5, column=0, padx=5, pady=5, sticky="w")


cpuLabel.grid(row=2, column=0, padx=5, pady=5, sticky="w")
cpu_bar.grid(row=2, column=1, padx=5, pady=5)
CPUPercentLabel.grid(row=2, column=2, padx=5, pady=5, sticky="e")
sys_load_bar.grid(row=5, column=1, padx=5, pady=5)

SwapLabel = ctk.CTkLabel(frm, text="Swap Usage", anchor="w", text_color="#cad3f5")
SwapLabel.grid(row=3, column=0, padx=5, pady=5, sticky="w")
swap_bar.grid(row=3, column=1, padx=5, pady=5)
SwapPercentLabel.grid(row=3, column=2, padx=5, pady=5, sticky="e")
virtual_memory_percent_label = ctk.CTkLabel(frm, text="0%", anchor="w", text_color="#cad3f5")

VirtualMemoryLabel = ctk.CTkLabel(frm, text="Virtual Memory Usage", anchor="w", text_color="#cad3f5")
VirtualMemoryLabel.grid(row=4, column=0, padx=5, pady=5, sticky="w")
virtual_memory_bar.grid(row=4, column=1, padx=5, pady=5)
VirtualMemoryPercentLabel.grid(row=4, column=2, padx=5, pady=5, sticky="e")

update_stats()

root.configure(fg_color='#24273a')
root.resizable(False, False)
root.mainloop()

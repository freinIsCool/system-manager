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

    cpu_bar.set(cpu_percent / 100)
    progress_bar.set(vm.percent / 100)
    storage_bar.set(disk.percent / 100)

    root.after(1000, update_stats)
    MemPercentLabel.configure(text=f"{vm.percent:.1f}%")
    StoragePercentLabel.configure(text=f"{disk.percent:.1f}%")
    CPUPercentLabel.configure(text=f"{cpu_percent:.1f}%")
    return (
        f"CPU: {cpu_percent}%, Memory: {vm.percent}%, Storage: {disk.percent}%"
    )

root = ctk.CTk()
root.title("System Manager")
root.geometry("275x125")

progStyle = ctk.CTkProgressBar(root, orientation="horizontal", width=300, height=50, corner_radius=10, border_width=2, border_color="#BDBDBD", progress_color="#4CAF50", fg_color="#E0E0E0")

frm = ctk.CTkFrame(master=root, fg_color="transparent") 
frm.pack(fill="both", expand=True)

MemLabel = ctk.CTkLabel(frm, text="Memory Usage", anchor="w", text_color="#cad3f5")
StorageLabel = ctk.CTkLabel(frm, text="Storage Usage", anchor="w", text_color="#cad3f5")
cpuLabel = ctk.CTkLabel(frm, text="CPU Usage", anchor="w", text_color="#cad3f5")

progress_bar = ctk.CTkProgressBar(frm, orientation="horizontal", width=120, height=20, corner_radius=9, fg_color="#363a4f", progress_color="#8aadf4")
storage_bar = ctk.CTkProgressBar(frm, orientation="horizontal", width=120, height=20, corner_radius=9, fg_color="#363a4f", progress_color="#8aadf4")
cpu_bar = ctk.CTkProgressBar(frm, orientation="horizontal", width=120, height=20, corner_radius=9, fg_color="#363a4f", progress_color="#8aadf4")

MemPercentLabel = ctk.CTkLabel(frm, text="0%", anchor="w", text_color="#cad3f5")
StoragePercentLabel = ctk.CTkLabel(frm, text="0%", anchor="w", text_color="#cad3f5")
CPUPercentLabel = ctk.CTkLabel(frm, text="0%", anchor="w", text_color="#cad3f5")

MemLabel.grid(row=0, column=0, padx=5, pady=5, sticky="w")
progress_bar.grid(row=0, column=1, padx=5, pady=5)
MemPercentLabel.grid(row=0, column=2, padx=5, pady=5, sticky="e")

StorageLabel.grid(row=1, column=0, padx=5, pady=5, sticky="w")
storage_bar.grid(row=1, column=1, padx=5, pady=5)
StoragePercentLabel.grid(row=1, column=2, padx=5, pady=5, sticky="e")

cpuLabel.grid(row=2, column=0, padx=5, pady=5, sticky="w")
cpu_bar.grid(row=2, column=1, padx=5, pady=5)
CPUPercentLabel.grid(row=2, column=2, padx=5, pady=5, sticky="e")

update_stats()

root.configure(fg_color='#24273a')
root.resizable(False, False)
root.mainloop()
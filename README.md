a simple, compact rust/gtk4 app that monitors the usage of storage, cpu, ram and more!
made to run in your corner


<img width="359" height="274" alt="preview of the app" src="https://github.com/user-attachments/assets/9b8582d7-c0a1-4ab3-913a-92946842976f" />

~~*fun fact: the entice code is located within the main.py file you can run it using python3 it will work the same as the appimage*~~

~~TODO: complete rust/gtk4 rewrite~~

### compiling from source

dependencies:
  + `gtk4 0.11.4+`
  + `sysinfo 0.39.6+`
  + `cairo-rs 0.22.0+`
  + `gdk 0.18.2+`
  + `rust`
  + `rustc`
  + `cargo`


```bash 
# 1. clone the repo
git clone https://github.com/freinIsCool/system-manager.git
cd system-manager

# 2. build
cd cargo/
cargo build
```
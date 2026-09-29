a simple, compact rust/gtk4 app that monitors the usage of storage, cpu, ram and more!
made to run in your corner


<img width="427" height="191" alt="dms_capture_1790689066568" src="https://github.com/user-attachments/assets/d2bb1bb9-149f-446c-9f28-efcede15bd3d" />





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
```

```bash
# 2. build
cd cargo/
cargo build
```
**or**

```bash
# build and run
chmod +x run
./run
```

the compiled binary is located in cargo/target/debug/SystemManager

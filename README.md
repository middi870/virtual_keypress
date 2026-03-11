# Virtual Keyboard Simulator (Rust + Linux uinput)

## Overview

This project implements a **virtual keyboard device in Rust** using the Linux **uinput subsystem**.
The program creates a software keyboard that the operating system treats exactly like a **real hardware keyboard**.

Using this virtual device, the program can:

* Simulate key presses
* Execute keyboard shortcuts
* Type commands automatically
* Trigger automation workflows

Example automation implemented in this project:

```
Super + Q → open terminal
type: echo "hi i am raj"
press Enter
```

---

# Architecture

The project is intentionally structured into multiple modules to mimic a layered automation system.

```
src/
 ├── key.rs      → low-level keyboard driver
 ├── skills.rs   → reusable automation skills
 └── main.rs     → execution logic
```

### key.rs

Responsible for:

* Creating the **virtual keyboard device**
* Sending **raw key events**
* Typing characters
* Sending key combinations

This module acts like a **software keyboard driver**.

---

### skills.rs

Contains reusable automation tasks such as:

* opening a terminal
* typing commands
* future system automation routines

Skills are higher-level behaviors built on top of the keyboard driver.

---

### main.rs

Defines the **actual automation sequence** executed by the program.

Example workflow:

```
create keyboard
wait
open terminal
wait
type command
press enter
```

---

# Linux Input Architecture

```
Rust program
     ↓
/dev/uinput
     ↓
Linux Input Subsystem
     ↓
Virtual Keyboard Device
     ↓
Window Manager / Desktop
     ↓
Focused application receives keys
```

Because this operates at the **kernel input layer**, applications cannot distinguish it from a physical keyboard.

---

# Requirements

Linux system with:

* Rust
* Cargo
* uinput kernel module

Install Rust if needed:

```
sudo pacman -S rust
```

---

# Enable uinput

Load the kernel module:

```
sudo modprobe uinput
```

Allow user access:

```
sudo chmod 666 /dev/uinput
```

To verify:

```
ls /dev/uinput
```

---

# Build the Project

```
cargo build
```

---

# Run

```
cargo run
```

The program will:

1. Create a virtual keyboard device
2. Open the system terminal
3. Type a command
4. Execute it automatically

---

# Example Output

```
Virtual keyboard ready
Opening terminal
Typing command
hi i am raj
```

---

# Debugging

If typing appears in the wrong terminal, run the program from another TTY.

Switch terminal:

```
Ctrl + Alt + F3
```

Run program:

```
cargo run
```

Return to GUI:

```
Ctrl + Alt + F1
```

---

# Future Improvements

Possible extensions:

### Full ASCII keyboard support

Add a keymap table:

```
HashMap<char, Key>
```

### Macro scripting engine

```
Action::Key
Action::Combo
Action::Type
Action::Wait
```

### Remote automation

Expose keyboard events via:

```
HTTP API
WebSocket
CLI commands
```

### Integration with sensors

Example:

```
WiFi presence detection
      ↓
Rust daemon
      ↓
virtual keyboard actions
```

---

# Educational Value

This project demonstrates:

* Linux input subsystem internals
* virtual device creation
* Rust systems programming
* automation architecture
* hardware abstraction layers

---

# License

MIT


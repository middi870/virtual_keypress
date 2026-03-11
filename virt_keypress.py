import uinput
import time

# Keys we will use
events = (
    uinput.KEY_LEFTMETA,
    uinput.KEY_Q,
    uinput.KEY_E,
    uinput.KEY_C,
    uinput.KEY_H,
    uinput.KEY_O,
    uinput.KEY_I,
    uinput.KEY_SPACE,
    uinput.KEY_ENTER,
    uinput.KEY_APOSTROPHE,
)

device = uinput.Device(events)

time.sleep(2)  # time to switch to desktop

# ---- open terminal (Super + Q) ----
device.emit(uinput.KEY_LEFTMETA, 1)
device.emit(uinput.KEY_Q, 1)
device.emit(uinput.KEY_Q, 0)
device.emit(uinput.KEY_LEFTMETA, 0)

time.sleep(1)

# ---- type echo "hi" ----
sequence = [
    uinput.KEY_E,
    uinput.KEY_C,
    uinput.KEY_H,
    uinput.KEY_O,
    uinput.KEY_SPACE,
    uinput.KEY_APOSTROPHE,
    uinput.KEY_H,
    uinput.KEY_I,
    uinput.KEY_APOSTROPHE,
]

for key in sequence:
    device.emit_click(key)
    time.sleep(0.05)

device.emit_click(uinput.KEY_ENTER)

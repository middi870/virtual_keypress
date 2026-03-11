#!/bin/bash
set -e

echo "This Project was created on Arch Linux"
echo "Use a Linux system for best experience."

if [[ "$(uname)" != "Linux" ]]; then
    echo "This script requires Linux."
    exit 1
fi

spin() {
    local pid=$!
    local delay=0.1
    local spinstr='|/-\'

    while ps -p $pid > /dev/null 2>&1; do
        local temp=${spinstr#?}
        printf " [%c]  " "$spinstr"
        spinstr=$temp${spinstr%"$temp"}
        sleep $delay
        printf "\b\b\b\b\b\b"
    done
}

echo "Loading uinput module..."
sudo modprobe uinput

echo "Setting permissions..."
sudo chmod 666 /dev/uinput

echo "Creating Python virtual environment..."
python3 -m venv venv &
spin
echo "Done."

echo "Activating virtual environment..."
source venv/bin/activate

echo "Installing python-uinput (this may take a moment)..."
pip install python-uinput >/dev/null 2>&1 &
spin
echo "Done."

cat > virt_keypress.py <<'EOF'
# Your program goes here
EOF

echo ""
echo "Setup complete."
echo ""
echo "Activate environment:"
echo "source venv/bin/activate"
echo ""
echo "Run program:"
echo "python virt_keypress.py"

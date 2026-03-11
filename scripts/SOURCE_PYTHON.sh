#!/bin/bash
set -e
echo "This Project was created on arch linux"
echo "So to run this Source_PYTHON.sh on a Linux machine,"
echo "as for seemless experience"
echo "Use Arch linux."

if [[ "$(uname)" != "Linux" ]]; then
    echo "This script requires Linux."
    exit 1
fi

spin() {
    local pid=$!
    local delay=0.1
    local spinstr='|/-\'

    while ps -p $pid > /dev/null; do
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

echo "Building project..."
echo "It may take time!!!"
spin()
# creating the venv
# echo "Creating Python virtual environment..."
python3 -m venv venv
#activation of venv
source venv/bin/activate

# echo "Installing dependencies (this may take a minute)..."
pip install python-uinput
# installing the package
echo "Done."

cat > virt_keypress.py <<'EOF'
EOF


echo ""
echo "Setup complete."
echo "Now activate the virtual-environment using"
echo "source venv/bin/activate"
echo ""
echo "Run the program using:"
echo "python virt_keypress.py"

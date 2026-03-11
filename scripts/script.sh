GREEN="\033[0;32m"
RESET="\033[0m"
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
echo -e "${GREEN}Installing dependencies...${RESET}"
echo "Creating Python virtual environment..."
python3 -m venv venv &
spin
echo "Done."

echo "Installing dependencies..."
pip install python-uinput &
spin
echo "Done."

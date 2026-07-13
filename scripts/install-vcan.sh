#!/usr/bin/env bash
# Install a persistent virtual CAN interface (survives reboot) via systemd.
#
#   sudo ./scripts/install-vcan.sh              # persistent vcan0
#   sudo ./scripts/install-vcan.sh vcan0 vcan1  # multiple
#
# Uninstall:  sudo systemctl disable --now vcan@vcan0
set -euo pipefail

if [[ ${EUID} -ne 0 ]]; then
    echo "error: run with sudo (installs a unit into /etc/systemd/system)" >&2
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
UNIT="${SCRIPT_DIR}/vcan@.service"

install -m 0644 "${UNIT}" /etc/systemd/system/vcan@.service
systemctl daemon-reload

ifaces=("$@")
[[ ${#ifaces[@]} -eq 0 ]] && ifaces=("vcan0")

for iface in "${ifaces[@]}"; do
    systemctl enable --now "vcan@${iface}.service"
    printf 'up: %s\n' "$(ip -brief link show "${iface}" 2>/dev/null || echo "${iface} (pending)")"
done

echo "Done — persistent across reboots. Verify: ip -details link show vcan0"

DESTINATION_BIN="$HOME/.local/bin/hypr_restore"
DESTINATION_SHARE="$HOME/.local/share/hypr_restore"

LISTENER_SERVICE="hypr-listener.service"
SNAPSHOT_SERVICE="hypr-snapshot.service"

rm -rf "$DESTINATION_BIN"
rm -rf "$DESTINATION_SHARE"

systemctl --user stop "$LISTENER_SERVICE" || true
systemctl --user disable "$LISTENER_SERVICE" || true

systemctl --user stop "$SNAPSHOT_SERVICE" || true
systemctl --user disable "$SNAPSHOT_SERVICE" || true

systemctl --user daemon-reload

rm -f -- "$HOME/.config/systemd/user/$LISTENER_SERVICE"
rm -f -- "$HOME/.config/systemd/user/$SNAPSHOT_SERVICE"
rm -f -- "$HOME/.local/bin/hyper-restore"
rm -f -- "$HOME/.local/bin/hyper-tui"
echo "Finished uninstalling"

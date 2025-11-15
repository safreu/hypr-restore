DESTINATION_BIN="$HOME/.local/bin/hypr_restore"
DESTINATION_SHARE="$HOME/.local/share/hypr_restore"

mkdir -p "$DESTINATION_BIN"
mkdir -p "$DESTINATION_SHARE"
mkdir -p ~/.config/systemd/user/

cp ./target/release/listener "$DESTINATION_BIN"
cp ./target/release/snapshot "$DESTINATION_BIN"
cp ./target/release/restore "$DESTINATION_BIN"
cp ./target/release/tui "$DESTINATION_BIN"

cp service/hypr-listener.service "$HOME/.config/systemd/user/"
cp service/hypr-snapshot.service "$HOME/.config/systemd/user/"

systemctl --user daemon-reload
systemctl --user enable hypr-listener.service
systemctl --user start hypr-listener.service

systemctl --user enable hypr-snapshot.service

ln -sf "$DESTINATION_BIN/restore" "$HOME/.local/bin/hypr-restore"
ln -sf "$DESTINATION_BIN/tui" "$HOME/.local/bin/hypr-tui"

echo "Finished installing"

# hypr-restore

Hypr-restore is a tool that tracks your currently open applications and restores them after a reboot.
It is primarily designed to run as a background systemd service, but it can also be used via the CLI.
Hypr-restore listens to [Hyprland’s IPC](https://wiki.hypr.land/IPC/) events and tracks opened, moved,
and closed applications by writing their state into a database.
After your system boots, the `hypr-snapshot.service` creates a snapshot of this database
so your applications can be restored to the workspaces they were previously located on.
After creating the snapshot, the service clears the database so it can begin tracking the new session.

## Table of Contents

- [Built With](#built-with)
- [Requirements](#requirements)
- [Features](#features)
- [Installation](#installation)
- [Uninstalling](#uninstalling)
- [Usage](#usage)
- [Ignoring Applications](#ignoring-applications)
- [Known Issues](#known-issues)
- [TUI](#tui)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

### Built With

![Rust Edition](https://img.shields.io/badge/rust-2024-orange?logo=rust)

### Requirements

- Rust
- Hyprland
- hyprctl
- systemd

## Features

- Tracks open applications in real time using Hyprland IPC
- Restores applications after reboot
- Provides both a command-line interface and a background service
- Restores applications to their original workspaces
- Provides a TUI to manage everything about hypr-restore [WIP]

## Installation

Use the package manager [cargo](https://crates.io/crates/hypr-restore) to install hypr-restore.

```bash
cargo install hypr-restore
```

Installing the crate only places the binaries. To fully set up hypr-restore, run:

```bash
hypr-restore install
```

This command sets up hypr-restore as a user service and creates everything it needs.

## Uninstalling

To remove the changes made by `hypr-restore install`, run:

```bash
hypr-restore uninstall
```

After this you can use

```bash
cargo uninstall hypr-restore
```

This removes the binaries installed with cargo.

## Usage

To run the event listener from your command line, use

```bash
hypr-restore listen
```

Note: Running this command manually while the `hypr-restore.listener` service is active will cause duplicate event tracking.

To snapshot the current database manually, use

```bash
hypr-restore snapshot
```

Normally this happens automatically for you after rebooting, via `hypr-snapshot.service`.

To restore the tracked applications, use

```bash
hypr-restore
```

To disable the `hypr-listener.service`, run

```bash
hypr-restore disable
```

To enable the `hypr-listener.service`, run

```bash
hypr-restore enable
```

To see a short description for each command, run

```bash
hypr-restore --help
```

To open the tui, run

```bash
hypr-restore tui
```

The update command is still WIP.

## Ignoring Applications

If you have applications that are automatically launched after rebooting through your Hyprland config,
you can add the `hypr-restore disable` before and the `hypr-restore enable`  after your exec-once to prevent them from being tracked:

```bash
exec-once = hypr-restore disable
exec-once = $terminal
exec-once = hypr-restore enable
```

If you want to permanently ignore certain applications, you can add their class names in lowercase, obtained via:

```bash
hyprctl clients
```

into the `~/.local/share/hypr_restore/classes.ignore` file.
Changes to this file are automatically detected and reloaded.

### TUI

The TUI currently allows you to:

- Inspect the database and snapshot
- View ignored classes and executable paths
- Check the status and logs of the listener and snapshot services
- (WIP) Create custom “workbenches”, which are predefined sets of applications/workspaces

### Known issues

- `JetBrains IDEs`: continuously send open/close window events, which may cause issues.

### Roadmap

- [ ] Implement a TUI to manage configuration, installation, and monitoring.
- [ ] Implement the update command
- [ ] Improve the command help section

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)

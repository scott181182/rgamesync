# Rgamesync

Rgamesync is an in-development program for syncing game save data between devices using cloud storage and [`rclone`](https://rclone.org/).

## Configuration

Rgamesync uses a single TOML file to configure the following:

- What game stores you use (e.g. Steam or Epic Games)
- What Rclone remote should be used for syncing
- What games should be synced, and how

By default this is located in your system's default configuration directory:

- Windows: `%USERPROFILE%\AppData\Roaming\rgamesync\rgamesync\config\config.toml`
- MacOS: `$HOME/Library/Application Support/com.rgamesync.rgamesync/config.toml`
- Linux: `$XDG_CONFIG_HOME/rgamesync/config.toml` (e.g. `~/.config/rgamesync/config.toml`)

You can pass in `-c/--config <config-path>` to the program to specify another configuration file.

## Remote Structure

Rgamesync syncs game data to your Rclone remote in a directory that should only be used by Rgamesync. By default this directory is `rgamesync/` at the root of the remote, but this can be changed with `remote.path` in your configuration file.

The structure that Rgamesync creates and uses (or plans to) on the remote looks like the following:

```
rgamesync/ (or custom)
 |- games/
     |- <game name>/
         |- sync/
         |   |- <...save files...>
         |- backup/
             |- <device name>/
                 |- <timestamp>/
                     |- <...save files...>
```

## Roadmap

- Solidify configuration structure
- Add interactive configuration creation (`rgamesync config`)
- Actually sync data using Rclone CLI (`rgamesync sync`)
- Add CI to run tests
- Add CD to make releases for different platforms (i.e. Windows, MacOS, and Linux)
- Eventually switch to using `librclone`, bundled with the application

## Inspirations

- [OpenCloudSaves](https://github.com/DavidDeSimone/OpenCloudSaves)

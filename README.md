# RURL - remote URL opening

Simple system to enable opening URLs from a remote session on a local client
machine.

The solution consists to two parts:

- `rurls` - server running on the client machine (where the URL will be
opened). This assumed to be a Mac and uses `open` to open the URL.
- `rurlc` - client to run in the remote session.

The default port is `7878`.

## Setup

1. Compile the server and client for the relevant machines.
1. Put the binaries in the paths.
1. On the client Mac, configure `launchd` to be running:
    - Copy the launch plist from `rurls` to `~/Library/LaunchAgents`
    - Adjust the `rurls` path as needed.
    - Load: `launchctl load ~/Library/LaunchAgents/org.lieder.rurls.plist`
    - Start: `launchctl start org.lieder.rurls`
    - Stop: `launchctl stop org.lieder.rurls`
    - Unload: `launchctl unload ~/Library/LaunchAgents/org.lieder.rurls.plist`
1. Set up a forwarded from port from the remote session to the client.



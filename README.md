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
1. Set up a forwarded from port from the remote session to the client.


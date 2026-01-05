# RURL - remote URL opening

Simple system to enable opening URLs from a remote session on a local client
machine.

The solution consists to two parts:

- `rurls` - server running on the client machine (where the URL will be
opened). This assumed to be a Mac and uses `open` to open the URL.
- `rurlc` - client to run in the remote session.

The default port is `7878`.

## Security

`rurls` includes URL validation to prevent command injection attacks from remote machines:

- **Default mode**: Only allows `http://` and `https://` URLs
- **Permissive mode**: Use `--allow-all-schemes` to allow all valid URL schemes (`file://`, `ftp://`, custom schemes, etc.)
- **Always protected**: Command flag injection (inputs starting with `-`) is blocked in all modes

### Command Line Options

```bash
# Start server with default settings (http/https only, port 7878)
rurls

# Custom port
rurls --port 8080
rurls -p 8080

# Allow all valid URL schemes (use only with trusted remote machines)
rurls --allow-all-schemes

# Combine options
rurls --port 8080 --allow-all-schemes

# Show help
rurls --help
```

**Recommendation**: Use default mode for untrusted remote machines. Only use `--allow-all-schemes` when you control and trust the remote machine.

## Setup

1. Compile the server and client for the relevant machines.
    - Manually:
        - `cargo build`
        - Put the binaries in the paths.
    - Using cargo install:
        - `cargo install --git https://github.com/FrankTaylorLieder/rurl`
        - This will install both binaries in the standard Cargo path.
1. On the client Mac, configure `launchd` to be running:
    - Copy the launch plist from `rurls` to `~/Library/LaunchAgents`
    - Adjust the `rurls` path as needed.
    - (Optional) Add `--allow-all-schemes` or other flags to the ProgramArguments array in the plist
    - Load: `launchctl load ~/Library/LaunchAgents/org.lieder.rurls.plist`
    - Start: `launchctl start org.lieder.rurls`
    - Stop: `launchctl stop org.lieder.rurls`
    - Unload: `launchctl unload ~/Library/LaunchAgents/org.lieder.rurls.plist`
1. Set up a forwarded port from the remote session to the client.
    - e.g. ssh config: `RemoteForward 7878 localhost:7878`

## Editor integration

To get `nvim` to use this for the `gx` command, add the following:

```
-- Enable remote URL opening on an SSH session
-- See: https://github.com/FrankTaylorLieder/rurl
if vim.env.SSH_CONNECTION then
  vim.ui.open = function(path)
    vim.fn.jobstart({ "rurlc", path }, { detach = true })
  end
end
```


# rmcache (rmc)

A very simple but powerful tool to delete cache files and directories.

### Features
`rmcache` is a utility designed to clear cache directories and execute cleanup commands based on a user-configurable setup. The utility offers the following features:

- **Shell Completions**: Generates shell completions for supported shells (Bash, Zsh, Fish, PowerShell, Elvish).
- **Configurable Paths**: Specify directories to clear using a configuration file.
- **Custom Commands**: Define custom shell commands to execute as part of the cleanup process.
- **Selective Cleaning**: Option to clean only specified caches or exclude specific caches from being cleaned.
- **Dry Run Mode**: Option to simulate the cleaning process without actually removing any files.

### Usage

#### Shell Completions

To generate shell completions for your shell, use the `completion` subcommand. You can specify the shell using the `--shell` option, or the utility will attempt to detect your shell from the `SHELL` environment variable.

```sh
rmcache completions --shell bash
```

Supported shells are `bash`, `zsh`, `fish`, `powershell`, and `elvish`.

#### Cleaning Cache

The `clean` subcommand is used to clean the specified cache directories and execute cleanup commands. You can configure the paths and commands in a configuration file (`~/.config/rmcache.toml`).

```sh
rmcache clean
```

##### Options

- `--dry-run`: Simulate the cleaning process without removing any files.
- `--shell <shell>`: Specify the shell to use for executing commands (overrides config file setting).
- `--only <ids>`: Clear the cache for the specified IDs only (comma-separated list).
- `--disable <ids>`: Clear the cache for all IDs except the specified ones (comma-separated list).

### Example Usage

#### Generate Shell Completions

```sh
rmcache completions --shell zsh
```

#### Clean All Caches

```sh
rmcache clean
```

#### Clean Only Specified Caches

```sh
rmcache clean --only cache1,cache2
```

#### Clean All Except Specified Caches

```sh
rmcache clean --disable cache3
```

#### Dry Run

```sh
rmcache clean --dry-run
```

### Example `rmcache.toml`

> [!NOTE]
> This file should be added to `~/.config/rmcache.toml`. If the file is not present, it will be created automatically by the program.

```toml
# Configuration options
[options]
# The shell to use for executing commands (default: "bash")
shell = "fish"

# Specify cache IDs to disable. These IDs will not be cleared.
disable = ["VSCode (Deep)", "Gradle", "go"]

# Specify cache IDs to enable. Only these IDs will be cleared.
# If both 'only' and 'disable' are used, 'only' takes precedence.
# only = []

# Cache paths for different applications
[paths]
MacOS = ["/Users/jatin/Library/Caches"]

VSCode = [
  "/Users/jatin/Library/Application Support/Code/Cache/Cache_Data",
  "/Users/jatin/Library/Application Support/Code/CachedExtensionVSIXs",
  "/Users/jatin/Library/Application Support/Code - Insiders/Cache/Cache_Data",
  "/Users/jatin/Library/Application Support/Code - Insiders/CachedExtensionVSIXs"
]

"VSCode (Deep)" = [
  "/Users/jatin/Library/Application Support/Code/CachedData",
  "/Users/jatin/Library/Application Support/Code/Service Worker/CacheStorage",
  "/Users/jatin/Library/Application Support/Code - Insiders/CachedData",
  "/Users/jatin/Library/Application Support/Code - Insiders/Service Worker/CacheStorage"
]

Discord = ["/Users/jatin/Library/Application Support/Discord/Cache"]

Arc = [
  "/Users/jatin/Library/Application Support/Arc/ArchiveSnapshotCache",
  "/Users/jatin/Library/Application Support/Arc/ArchiveItemsFaviconCache"
]

Chrome = ["/Users/jatin/Library/Application Support/Google/Chrome/Profile 1/Service Worker/CacheStorage"]

Gradle = ["/Users/jatin/.gradle/caches"]

Bun = ["/Users/jatin/.bun/cache"]

# Custom commands for clearing caches
[commands]
npm = "npm cache clean --force"
go = "go clean -cache -modcache -testcache"
```

### Troubleshooting

- Ensure your configuration file (`~/.config/rmcache.toml`) exists and is correctly formatted.
- Use the `--dry-run` option to see which files and directories would be affected before performing the actual clean-up.
- For verbose output, increase the verbosity level by modifying your configuration or using appropriate command-line flags.

With `rmcache`, managing and cleaning up your cache directories becomes a straightforward and configurable process, tailored to your specific needs.
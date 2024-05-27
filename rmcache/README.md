# rmcache (rmc)

A very simple but powerful tool to delete cache files and directories.

### Features
1. **Cache Management**: Allows users to clear or print information about cached files and directories.
2. **Selective Clearing**: Provides options to clear the cache for specific IDs or to exclude certain IDs from clearing.
3. **Dry Run Mode**: Users can preview the files and directories that would be removed without actually deleting them.
4. **Shell Execution**: Supports executing custom shell commands for cache clearing.
5. **Verbose Logging**: Offers detailed logs for operations, helping with debugging and monitoring.
6. **Configuration Management**: Loads and stores configuration settings from a TOML file, allowing persistent user settings.

### Usage
The program is invoked from the command line and supports various options and flags for customizing its behavior. Below are the options available:

- **-c, --clear**: Clears the cache.
- **--dry-run**: Prints what would be removed without actually removing any files.
- **-s, --shell <SHELL>**: Specifies the shell to use for executing commands (e.g., bash, zsh).
- **-o, --only <ID>**: Clears the cache for the specified ID(s). Multiple IDs can be specified, separated by commas.
- **-d, --disable <ID>**: Clears the cache for all IDs except the specified ones. Multiple IDs can be specified, separated by commas.
- **-v, --verbose**: Prints verbose information about the operations being performed.

### Example Usage
1. **Clear the Cache**:
    ```bash
    ./rmcache -c
    ```
    This command clears the cache based on the configuration settings and the options provided.

2. **Dry Run**:
    ```bash
    ./rmcache --dry-run
    ```
    This command prints what would be removed without actually deleting any files or directories.

3. **Use a Specific Shell**:
    ```bash
    ./rmcache -c --shell zsh
    ```
    This command clears the cache using the Zsh shell instead of the default shell.

4. **Clear Cache for Specific IDs**:
    ```bash
    ./rmcache -c -o cache1,cache2
    ```
    This command clears the cache only for the specified IDs (`cache1` and `cache2`).

5. **Disable Clearing for Specific IDs**:
    ```bash
    ./rmcache -c -d cache1,cache2
    ```
    This command clears the cache for all IDs except `cache1` and `cache2`.

6. **Verbose Logging**:
    ```bash
    ./rmcache -c -v
    ```
    This command clears the cache and provides detailed logs of the operations being performed.

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

### Additional Information
- This file should be located at `~/.config/rmcache.toml`.
- If the file does not exist, it will be created automatically by the program with default settings.
- Comments in the TOML file explain the purpose of each section and setting.
- If both the `only` and `disable` options are used, the `only` option takes precedence, meaning only the specified IDs in `only` will be cleared, and `disable` will be ignored.
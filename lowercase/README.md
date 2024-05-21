## lowercase (lwc)

Simple and lightweight CLI tool to fight my OCD and convert filenames to lowercase. (also works with directories)

### Features

- **Lowercase Conversion**: Converts filenames and directories to lowercase. This operation is not recursive, meaning it only affects the specified directory and not its subdirectories.
- **Space Replacement**: Joins filenames or directories that contain spaces by replacing the spaces with a specified delimiter. By default, this delimiter is a hyphen (`-`).
- **Custom Delimiter**: Allows the user to specify a custom delimiter to replace spaces in filenames or directories.
- **Lowercase Only Option**: Provides an option to only convert filenames and directories to lowercase without replacing spaces.

### Usage

The tool accepts the following command-line arguments:

- `path`: The path to the file or directory you want to process.
- `-d, --delimiter <delimiter>`: The delimiter to use in file names. Default is a space (" ").
- `-r, --replacer <replacer>`: The character to use instead of a space. Default is a hyphen ("-").
- `-lo, --lowercase-only <lowercase_only>`: Whether to only lowercase the file name without replacing spaces. Default is `false`.

Example usage:

```bash
lwc --path /path/to/your/file --delimiter _ --replacer - --lowercase-only true
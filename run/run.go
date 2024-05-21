package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"unicode"

	"github.com/fatih/color"
	"github.com/jatindotdev/cli/lib"
)

const (
	JavaScript = "js"
	C          = "c"
	Cpp        = "cpp"
	Dart       = "dart"
	Swift      = "swift"
	Python     = "py"
	Java       = "java"
	Go         = "go"
)

func main() {
	args := os.Args[1:]

	if len(args) == 0 || lib.Contains(args[0], "help", "-h", "--help") {
		printHelp()
		os.Exit(0)
	}

	fileName := args[0]

	if !lib.FileExists(fileName) {
		fileNameWithoutDir := filepath.Base(fileName)
		fmt.Fprintf(os.Stderr, color.RedString("Error: `%s` does not exist!\n"), fileNameWithoutDir)
		os.Exit(1)
	}

	extension := filepath.Ext(fileName)
	fileNameWithoutExt := strings.TrimSuffix(fileName, extension)
	remainingArgs := []string{}

	for _, arg := range args[1:] {
		remainingArgs = append(remainingArgs, fmt.Sprintf("'%s'", arg))
	}

	remainingArgsAsString := strings.Join(remainingArgs, " ")

	commandsPerExt := map[string][]string{
		JavaScript: {fmt.Sprintf("bun run %s %s", fileName, remainingArgsAsString)},
		C: {
			fmt.Sprintf("gcc -std=c2x -o %s %s", fileNameWithoutExt, fileName),
			fmt.Sprintf("./%s %s", fileNameWithoutExt, remainingArgsAsString),
			fmt.Sprintf("rm -rf %s", fileNameWithoutExt),
		},
		Cpp: {
			fmt.Sprintf("g++ -std=c++17 -o %s %s", fileNameWithoutExt, fileName),
			fmt.Sprintf("./%s %s", fileNameWithoutExt, remainingArgsAsString),
			fmt.Sprintf("rm -rf %s", fileNameWithoutExt),
		},
		Dart:   {fmt.Sprintf("dart run %s %s", fileName, remainingArgsAsString)},
		Swift:  {fmt.Sprintf("swift %s %s", fileName, remainingArgsAsString)},
		Python: {fmt.Sprintf("python3 -u %s %s ", fileName, remainingArgsAsString)},
		Java: {
			fmt.Sprintf("javac %s", fileName),
			fmt.Sprintf("java %s %s", fileNameWithoutExt, remainingArgsAsString),
			fmt.Sprintf("rm -rf %s.class", fileNameWithoutExt),
		},
		Go: {
			fmt.Sprintf("go run %s %s", fileName, remainingArgsAsString),
		},
	}

	commands := commandsPerExt[extension[1:]]

	if commands == nil {
		fmt.Fprintf(os.Stderr, color.RedString("Error: we don't support `%s` extensions yet.\n"), extension)
		fmt.Fprintln(os.Stderr, "Please raise an issue here https://github.com/jatindotdev/cli/issues")
		os.Exit(0)
	}

	for _, command := range commands {
		cmdIndex := strings.Index(command, " ")
		cmdBin := command[:cmdIndex]

		// Split command and arguments
		inQuotes := false
		cmdArgs := strings.FieldsFunc(command[cmdIndex:], func(r rune) bool {
			if r == '\'' {
				inQuotes = !inQuotes
			}
			return unicode.IsSpace(r) && !inQuotes
		})

		// Remove single quotes from arguments
		for i, arg := range cmdArgs {
			cmdArgs[i] = strings.Trim(arg, "'")
		}

		cmd := exec.Command(cmdBin, cmdArgs...)

		// Set the standard streams
		cmd.Stderr = os.Stderr
		cmd.Stdout = os.Stdout
		cmd.Stdin = os.Stdin

		err := cmd.Run()
		if err != nil {
			continue
		}
	}
}

func printHelp() {
	color.Set(color.Bold, color.FgMagenta)
	fmt.Print("Run")
	color.Unset()
	fmt.Print(" does what its name says: it runs a file.\n\n")
	color.Unset()
	color.Set(color.Bold)
	fmt.Print("Usage: run <command>")
	color.Set(color.FgGreen)
	fmt.Println(" [...args]")
	color.Unset()
	fmt.Print("\n  [...args]     Arguments that will be passed to the command\n\n")
	color.Set(color.Bold)
	fmt.Println("Commands:")
	color.Unset()
	fmt.Println("  help          Show this help message")
}

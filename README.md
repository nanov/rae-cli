# Real Academia Española CLI Dictionary (rae-cli)

A command-line tool to search and display Spanish word definitions from the [Real Academia Española (RAE)](https://dle.rae.es/) dictionary.

<img src="https://github.com/nanov/rae-cli/blob/main/docs/rae-cli-demo.gif?raw=true" />


## Installation

### macOS - Homebrew
```sh
brew tap nanov/rae-cli
brew install rae-cli
```

### Linux - Community Support Needed
Support and testing are needed for building distribution-specific packages. Contributions and feedback on this are welcome.

### Windows - Community Support Needed
Support and testing are needed for packaging on Windows. Feedback on package managers and compatibility is appreciated.

Alternatively, download the latest release from [releases](https://github.com/nanov/rae-cli/releases), unzip, and place the executable in a directory included in your `PATH` environment variable for easy access.

## Usage

Search for word definitions from the Real Academia Española.

```sh
rae-cli <palabra>
```

### Arguments
- `<palabra>`: The word you want to search for.

### Options
- `-h`, `--help`: Display help.
- `-V`, `--version`: Display the version number.

If the word is found, the tool will display its definition. If suggestions are available, a menu will appear for you to select the desired word.
Looks like I sent that before I finished! Here’s the rest of the README:

---

## Tips

For words with lengthy definitions (especially verbs, as full conjugation tables are displayed), combining `rae-cli` with a pager like `less` can make it easier to navigate.

You can add this alias to your `.bashrc` or `.zshrc` file to automatically use `less` for pagination:

```sh
function rae_cli () { rae-cli "$@" | less -F }
alias rae=rae_cli
```

With this setup, typing `rae` will call `rae-cli` with `less` by default, allowing you to scroll through longer entries easily. 

---


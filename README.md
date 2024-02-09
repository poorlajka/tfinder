# Termfinder - A minimal, cross-platform, tui based file manager written in Rust  

### About 

### Features
* Mouse support
* Vim esque bindings
* TODO: Image displaying
* Custom styling
* TODO: Custom file commands
* Works straight in tty with no wm


**Warning! Many features missing or likely not tested properly. Don't use for any actual stuff, this is just a toy-project for now so use with caution.**

## Installation

To install termfinder simply clone the repo and build with Cargo:

```shell
git clone git@github.com:poorlajka/termfinder.git
cd termfinder
cargo build --release
```

## Running

```shell
cd release
./termfinder
```

Optionally move the binary to a desired location like for example:

```shell
mv termfinder /usr/local/bin/
```

## Usage


### Keybinds:

* up/down/arrow or j k to move up down in the list of files in the currently selected file pane.
* left/right arrow or h l to 
* Esc to quit
* Enter to go into folder

### Mouse

Try clicking on stuff UwU

## Configuration

termfinder will look for a config file at

```shell
~/.config/termfinder/termfinder.toml
```

Note: This file and it's parent directory have to be created manually.

## Setting colors

**termfinder consists of:**

* Two file panels: one to the far left and the second one just beside it
* A image/info display panel to the far right
* A path trail at the top
* A command bar/prompt at the bottom

You can set the colors of these components like:

```
[colors]
global {
    background = "black"
}
file_panes = {
    background = "lightblue",
    border = "red",
    hover = "#ffffff",
    selected = "magenta",
    text_default = "white",
    text_selected = "#aaaaaa"
}
img_pane = {
    background = ""
}
path_trail = {
    background = ""
}
prompt_bar = {
    background = ""
}
```

## Adding custom commands

```
[commands]
commands = [
    "T tar -xvf",
    "U unzip",
    "V vim",
    "G grep"
]
```
# Mystical Sphere ("Magic 8-ball")

![Demo gif](./assets/demo.gif)

I wanted to make something kind of silly to learn more about Rust and this is what I came up with. A multi-platform, multi-front-end toy program that emulates the behavior of a Magic 8-ball, but configurable!

## Why?

Why not? I wanted to learn more about cargo workspaces and this is the excuse I came up with to do that. Also, as ChatGPT and the like has shown us, having life's burning questions answered arbitrarily by a machine is entertaining.

## Contributing

Right now this project is in it's early days and a bit in flux, so I wouldn't recommend taking the time to contribute anything until things are a little more stable.

## Platforms

As I mentioned, I plan on releasing for multiple platforms and front-ends. As of 2023-02-27 I only have a simple CLI defined with more on the horizon.

### Terminal

#### CLI & TUI

This is the most basic form of Mystical Sphere. By default, running the program will emulate a simple conversation in which the sphere introduces itself to the user and prompts them to ask it a yes or no question which the program provides a vague answer to. The conversational element can be bypassed using a simple flag (`-q`, `--quiet`), ideal if one would like to use this program's output someplace else.

**Coming soon:** A command that launches the program in TUI mode!

##### Usage

```txt
Usage: mystical-sphere [OPTIONS]

Options:
  -c, --config <CONFIG>  Path to alternate config [default: ~/.config/mystical-sphere/config.toml]
  -q, --quiet            If present, the program will only output a random answer and quit
  -h, --help             Print help
  -V, --version          Print version
```

### Web

**Coming soon!**

### Desktop

**Coming soon!**

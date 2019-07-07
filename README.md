![icon](icon.png)

## DIRTY
toolkit for things

### modules
- **window** Window Creation & Config
- **gfx** GPU Accelerated Drawing
- **img** Image Loading & Writing
- **audio** Sound Loading & playback
- **fs** Common File System Functions
- **http** Simple HTTP Client & Server
- **term** TUI Utilities
- **col** Common Collision Detections
- **ase** Load Aseprite Spritesheets

This library was intended to be a game toolkit, but added a lot of other stuff due to my other scripting needs, and to provide a more integrated and unified scripting interface. All the modules can be configured with cargo feature:
```toml
default = [ "lua", "fs", "gfx", "img", "audio", "http", "term", "col", "ase", ]
```
All the modules can be used with lua or as rust modules, toggle with `lua` feature

### cli

The `dirty` binary is for running lua scripts
```sh
$ dirty frog.lua
```

If no argument is provided, it'll search for `main.lua`. It'll search everywhere possible, including `${Bundle}/Contents/Resources/` on MacOS, making it easy for packaging windowing applications.

### facts
- `DIRTY` is short for **Dangerous Ichthyopolist Reincarnates Tropical Yeti**


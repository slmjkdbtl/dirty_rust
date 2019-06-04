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

all the modules have lua bindings, enable with `lua` feature

please use cargo feature to configure what you need, otherwise this package is going to be huge
```toml
default = [ "lua", "fs", "gfx", "img", "audio", "http", "term", "col", "ase", ]
```

### status
**10%** complete. The code is not robust in any way. Meant to be my personal library for doing things, will not be a serious library for production use ever.


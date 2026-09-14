<div align="center">
  <img src="assets/logo.png"/>
  <h1 align="center">Meadows FS</h1>

  <img src="https://img.shields.io/github/actions/workflow/status/TeamMeadows/fs/build.yml">
  <img src="https://img.shields.io/github/release/TeamMeadows/fs.svg">
  <img src="https://img.shields.io/github/issues/TeamMeadows/fs.svg">
  <img src="https://img.shields.io/github/license/TeamMeadows/fs.svg">

[<kbd> <br> Download <br> </kbd>][Download]
</div>

[Download]: https://github.com/TeamMeadows/fs/releases/latest
## About
Meadows FS is a binary module that provides functions for writing to and reading from files in your server's folder.

### Example
```lua
require("meadows_fs")

local MeadowsFS = atomic.package.get("team.meadows.fs", "*")
---@cast MeadowsFS MeadowsFS

local file = MeadowsFS.create("garrysmod/addons/.gitignore")

file:write("/hls-system")
assert(file:read() == "/hls-system")

local isOk = MeadowsFS.remove("garrysmod/addons/.gitignore")
print(isOk)
```

### Installation
1. Install [Atomic Framework](https://github.com/TeamMeadows/atomic-framework)
2. Download Meadows FS from the table below and put downloaded file in `garrysmod/lua/bin/`. 
## Downloads
| Architecture | Download                                                                                           |
|--------------|----------------------------------------------------------------------------------------------------|
| Windows `x32`  | [Download](https://github.com/TeamMeadows/fs/releases/latest/download/gmsv_meadows_fs_win32.dll)   |
| Windows `x64`  | [Download](https://github.com/TeamMeadows/fs/releases/latest/download/gmsv_meadows_fs_win64.dll)   |
| Linux `x32`    | [Download](https://github.com/TeamMeadows/fs/releases/latest/download/gmsv_meadows_fs_linux.dll)   |
| Linux `x64`    | [Download](https://github.com/TeamMeadows/fs/releases/latest/download/gmsv_meadows_fs_linux64.dll) |
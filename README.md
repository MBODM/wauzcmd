# wauzcmd
A tiny command line unzip tool for World of Warcraft addons 

![WAUZ](screenshot.png)

### What it is

This is the command line version of [WAUZ](https://github.com/mbodm/wauz). It works in exactly the same way as his "big brother" does. It is a typical command line executable for Windows, written in Rust.

### How it works

Exactly like WAUZ. Instead of selecting the 2 folders in a GUI, just call _wauzcmd.exe_ and pass the 2 folders as command line arguments.

A few notes here:
- Use absolute folder paths.
- Use double quotes ("") if a folder path contains spaces.
- Both folders have to exist before calling the executable.

For more information have a look at the [WAUZ](https://github.com/mbodm/wauz) page.

### Why it exists

Because i promised in the "[Notes](https://github.com/mbodm/wauz#notes)" section of the WAUZ page:

>There will be also a command line version of WAUZ. Soon.

And here it is.

### Requirements

- 64-bit Windows

There are not any other special requirements. It is written in Rust and the release binaries are natively compiled for the Windows x64 platform.

#### Have fun.

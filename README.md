# mywm
mywm is a simple tiling window manager built using the rust crate: [penrose](https://github.com/sminez/penrose).

## Build
Mywm can be build using with cargo using the command:
```bash
cargo build --release
```

## Configuration
The terminal, application launcher, and start-up script are configurable through environment variables.
Simply export the following variables through whichever method you prefer, I put them in my .zshenv.
```
export MYWM_TERMINAL=<terminal>
```
```
export MYWM_LAUNCHER=<launcher>
```
```
export MYWM_START_SCRIPT=<path_to_script>
```

## Jetbrains IDEs
Like many tiling-window managers, Jetbrains IDEs do not function properly without exporting the variable:
```
export _JAVA_AWT_WM_NONREPARENTING=1
```

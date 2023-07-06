# rust_physics_engine

## ChangeLog

# Introduction
The `SDL2` library is natively written in `C` that came bare-bone. We will
install the additional modules that is necessary to our basic graphics for
current objectives.

## Project requirements

We will create several modules for the `physics engine` including
1. Animation platform
	1. Need to define the animation screen basis.
		1. width, height
		2. refresh rate (FPS)
		3. time-delta calculation (using the actual vs computed and compare between them).
	2. Need to include the grid line
		1. Major grid lines
		2. Minor gird lines
	3. Centerlines object with
		1. Major ticks
		2. minor ticks
		3. font label of the coordinates.
2. Animation object class
	1. Including the main object mover
3. Vector class  - 2D at first
	1. Basic constructor
	2. Vector methods
4. Logging and debugging class
5. Conceptualized module system for drawing session (chapters based )
6. Graphics configurations
	1. Color struct
	2. Font struct
 7. Graphics elements - fundamental
	 1. Arrow, with tracer for the object moving
	 2. Circle with edge circle
	 3. Zoom scale


## Requirements
- `SDL2` has several extensions such as ("ttf","image","gfx","mixer"), in
addition there is a module which I will not use called `network` under the sdl2
too.
    - `sdl2`      : is the back-end library that create the premative-graphics.
    - `sdl2-image`: which will handle import images to our current graphics.
    - `sdl2-tff`  : to handle the text and adding text
    - `sdl-mixer` :
    - `sdl2_gfx`  : this will offer us to draw circles, fill-cricles, arches ..etc.
- Use the homebrow formula for `M1 Arm64`
```sh
arch -arm64 brew install sdl2\
                         sdl2_image\
                         sdl2_ttf\
                         sdl2_mixer\
                         sdl2_gfx\
```
- Verify the installed Requirements
```sh
╰─ sdl2-config --libs
-L/opt/homebrew/lib -lSDL2
╭─ gmbp  󰇄 GMacBookPro on ~/Desktop/devCode/rust_fundamentals/rust_graphics_engine   
├─󰚩 INSERT 󰇋 19531d8h0m|master ?6
╰─ pkg-config --libs SDL2_ttf
-L/opt/homebrew/Cellar/sdl2_ttf/2.20.2/lib -L/opt/homebrew/lib -lSDL2_ttf -lSDL2
╭─ gmbp  󰇄 GMacBookPro on ~/Desktop/devCode/rust_fundamentals/rust_graphics_engine   
├─󰚩 INSERT 󰇊 19531d8h3m|master ?6
╰─ pkg-config --libs SDL2_mixer

-L/opt/homebrew/Cellar/sdl2_mixer/2.6.3_1/lib -L/opt/homebrew/lib -lSDL2_mixer -lSDL2
╭─ gmbp  󰇄 GMacBookPro on ~/Desktop/devCode/rust_fundamentals/rust_graphics_engine   
├─󰚩 INSERT 󰇏 19531d8h3m|master ?6
╰─ pkg-config --libs SDL2_image

-L/opt/homebrew/Cellar/sdl2_image/2.6.3_1/lib -L/opt/homebrew/lib -lSDL2_image -lSDL2
```
## Project Build
### Build in Debug Mode

```rust
cargo run
# or
RUST_BACKTRACE=1 cargo run --quiet
```
### Build in Release Mode
- To build the project we use

```rust
cargo build --release
```
or
```rust
cargo run --release
```
## Checking the project stcture
- We can use the `modules`

```rust
cargo install cargo-modules
cargo modules generate tree --with-types
```

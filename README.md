# Torus

Torus is a command-line tool for rendering a rotating torus (donut shape) in your terminal using ASCII characters.

## Features
- Fully configurable torus dimensions (major/minor radii).  
- Adjustable camera distance, terminal width, height and more.  
- Simple angle increments for dynamic rotation.  
- Offsets to shift the rendered donut anywhere in the terminal.  

## Installation

1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone this repository: 
 ```bash
   git clone https://github.com/amrosia/torus.git
   ```
3. Build the project:
``` 
cd torus
cargo build --release
```
The compiled binary will be located at:
```
target/release/torus-ascii
```
After building the project, you can run the compiled binary:
```
./torus-ascii [OPTIONS]
```
## Options

    --major <MAJOR_R>
    The distance from the center to the donut (major radius).
    Default: 2.0

    --minor <MINOR_R>
    The size of the donut (minor radius).
    Default: 1.0

    -d, --dist <CAM_DISTANCE>
    The distance from the camera to the donut.
    Default: 5.0

    --height <HEIGHT>
    The height of the terminal in rows.
    Default: 24

    --width <WIDTH>
    The width of the terminal in columns.
    Default: 80

    --delay <DELAY>
    The delay (in milliseconds) between each frame.
    Default: 20

    --ring <DONUT_RING_ANGLE>
    The angle of the donut ring, in degrees.
    Default: 360.0

    --tube <DONUT_TUBE_ANGLE>
    The angle of the donut tube, in degrees.
    Default: 360.0

    --x-inc <X_INC>
    X-axis increment angle.
    Default: 0.04

    --z-inc <Z_INC>
    Z-axis increment angle.
    Default: 0.02

    --offset-x <OFFSET_X>
    Horizontal offset for the donut.
    (optional)

    --offset-y <OFFSET_Y>
    Vertical offset for the donut.
    (optional)

## Examples

Render a torus with default settings

```torus-ascii```

Render a torus with a larger camera distance and slower rotation

```torus-ascii --dist 7 --delay 50 ```

## Contributing

Contributions, issues, and feature requests are welcome. Feel free to check issues or open a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

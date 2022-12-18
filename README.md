![Banner](https://s-christy.com/status-banner-service/physics-graph/banner-slim.svg)

## Overview

This is a graphing program that uses physics to determine the layout of edges
and nodes. There is a strict separation between the part of this program that
calculates layout and the renderer. This will allow for multiple rendering
backends to be added in the future.

## Features

- Renders directed graphs to SVG images
- Layout determined by gravitational (inverse square law) and spring (Hooke's law) physics
- Edges and nodes can be individually styled
- Simple and intuitive input language
- A normalization step fits output to image bounds
- A traversal algorithm is used to ensure that every node is connected
- Invisible edges can be added to manipulate structure
- Adjustable edge weights
- Example scripts and their output
- Multi-line node names
- Text dimensions with `rusttype`
- Deterministic layout

## Usage and Input Language

```
cargo run -- path/to/file.graph
```

Output image will be `out.svg`

The input specification language is still under development. Please refer to the
examples to see current usage.

## Gallery

<img src="./screenshots/complex.svg" width=250></img>
<img src="./screenshots/random.svg" width=250></img>
<img src="./screenshots/circle.svg" width=250></img>
<img src="./screenshots/pie.svg" width=250></img>
<img src="./screenshots/quad.svg" width=250></img>
<img src="./screenshots/simple.svg" width=250></img>
<img src="./screenshots/tri.svg" width=250></img>

## License

This work is licensed under the GNU General Public License version 3 (GPLv3).

[<img src="https://s-christy.com/status-banner-service/GPLv3_Logo.svg" width="150" />](https://www.gnu.org/licenses/gpl-3.0.en.html)

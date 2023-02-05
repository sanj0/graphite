# Graphite
Find out visually by creating nodes, links and dragging them around!

## Build & Run
Have rust and cargo installed, clone the repo and run `cargo r [--release]`.

## Usage
In the empty window before you, click to create nodes, click two nodes in a row
to create a link between them (and again to delete the link) and drag them around
to see if your graph is planar (or to just play around a bit).

## TODO
- Make label generation not panic after 26 letters of the alphabet
- pretty-fy the graphics
- Implement simple file format to load graphs (that are given only by sets of
  nodes and links)


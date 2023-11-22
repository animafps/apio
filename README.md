# apio (name not finalised)
*this readme is the current planning document*

A video processing workflow framework with plugable filters and a dynamic filter graph

Think gstreamer simplified for processing on the raw frames only with first class Rust, C support for plugins

With backwards compatibility with vs plugins (stretch goal)

## Motovation

New tools and workflows for video creation is so scattered and things like vapoursynth having a list of issues that makes development on projects like teres slow and inefficient 

## Plugins

Main thing to integrate and make posible but not limited to:

- Interpolation
- Upscaling
- Blending

## Organisation

### Core (apio subfolder)

- Holds the main runtime code
- Main plugins nested

### apio-plugin

- Api for plugins
- Plugin helpers/macros

### Lib (maybe)

- api/ffi for runtime
- describes filter graph dynamics and process
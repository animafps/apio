# apio (name not finalised)
*this readme is the current planning document*

A video processing workflow framework with plugable filters and a dynamic filter graph

Think gstreamer simplified for processing on the raw frames only with first class Rust, C support for plugins

As well for every component to be modular and indepentent of each other except for the executeable part which is opionated by design

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
- executable

### apio-plugin

- Api for plugins
- Plugin helpers/macros

### Lib

- api/ffi for runtime
- describes filter graph dynamics and process


## Filter Graph


```
[input] --> filter --> [output]
```

```
[input] --> splitfilter [pad1,pad2] --> [pad1]
                                 |
                                 |
                                 + --> [pad2] 
```

```
"filter[pad1];[pad2]filter2

[input] --> filter[unnamed output pad] --> [unnamed inputpad]filter2 --> [output]
                       |
                       |
                       + 
```

- if there is no connection of the named pads, will connect to the next filter defined
- like that if there is a connection there is no default pads defined
- the filter graph must be complete (there exists a complete path from input to output)
- the input and output are special filters with no input or output pads respectfuly 
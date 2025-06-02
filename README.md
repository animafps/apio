# APIO: A Modern FFmpeg-Based Video Filter Framework
*note: using this as a planing document*

## Overview

APIO is a video filtering and processing framework built with Rust and FFmpeg, designed to provide a simple yet powerful interface for video processing workflows. The current MVP focuses on core filter graph functionality with plans for future scalability enhancements.

## Current Status: MVP

The Minimum Viable Product includes:

- **JSON Filter Graph Parsing**: Define filter chains using JSON configuration
- **Single Host Operation**: Optimized for single-machine processing
- **Core Filter Support**: Basic filtering operations without encoding complexity
- **FFmpeg Integration**: Direct libavfilter API usage for maximum compatibility
- **Sample Filters**: Demonstration filters for testing and development

## Getting Started

### Prerequisites

- Rust 1.70+
- FFmpeg 4.4+

### Installation

```bash
git clone https://github.com/animafps/apio.git
cd apio
cargo build --release
```

### Basic Usage

```bash
# Process video with JSON filter configuration
./apio process input.mp4 --config filters.json --output output.mp4
```

### Sample Filter Configuration

```json
{
  "filters": [
    {
      "type": "scale",
      "params": {
        "width": 1920,
        "height": 1080
      }
    },
    {
      "type": "blur",
      "params": {
        "radius": 5
      }
    }
  ]
}
```

## Architecture (Current)

### Core Engine (Rust)

The current implementation provides:

- FFmpeg libavfilter API wrapper
- Basic filter graph construction and management
- AVFrame memory management
- Simple filter chaining

### Filter Graph Implementation

- Directed Acyclic Graph (DAG) structure
- Frame reference passing between filters
- Minimal memory allocation for format-compatible operations
- Independent filter graph management from LibAV

## Benchmarks

Performance testing focuses on:

- **Rust + FFmpeg vs VapourSynth + FFmpeg** for identical operations
- **Memory usage** and **throughput** metrics for core operations
- **Filter chain performance** with various complexity levels

## Planned Enhancements

### Scripting Layer (Future)
- Lua integration via mlua for high-level API
- Intuitive filter chaining syntax
- Plugin auto-completion support

Example planned syntax:
```lua
local output = clip("input.mp4")
  :filter(blur({ radius = 5 }))
  :filter(scale({ width = 1920, height = 1080 }))

output:write("out.mp4")
```

### VapourSynth Compatibility (Future)
- Plugin bridge for VS compatibility (Rustsynth)
- Dynamic plugin loading
- Filter metadata exposure

### Encoding Integration (Future)
- Scene-based encoding workflows
- Per-chunk programmable parameters
- GOP-aware processing

### Scalability Features (Future)

#### Multi-Core Support
- Rayon-based parallelization
- Frame range splitting
- Parallel filter graph instantiation

#### Distributed Processing
- Scene-based chunking
- Object storage integration (S3/MinIO)
- Kubernetes orchestration
- Redis-based work queues

### Performance Optimizations (Future)
- Zero-copy frame pools
- Memory-mapped I/O
- SIMD optimization
- GPU acceleration (CUDA/VAAPI)

### Tooling & Deployment (Future)
- Docker containerization
- Web-based visual editor
- Helm charts for Kubernetes
- CI/CD pipeline integration

## Contributing

We welcome contributions to help build towards our vision! Current areas where help is needed:

- Core filter implementations
- JSON schema validation
- Performance optimizations
- Documentation and examples

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Development Roadmap

1. **Phase 1 (Current MVP)**: Core filter graph functionality
2. **Phase 2**: Lua scripting integration and basic encoding
3. **Phase 3**: Multi-core processing and plugin system
4. **Phase 4**: Distributed processing capabilities
5. **Phase 5**: Advanced tooling and GPU acceleration

## License

This project is licensed under the GPL 3.0 License - see the [LICENSE](LICENSE) file for details.
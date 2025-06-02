# Contributing to APIO

Thank you for your interest in contributing to APIO! We welcome contributions from developers of all skill levels. This document provides guidelines for contributing to the project.

## Getting Started

### Development Environment Setup

1. **Install Prerequisites**
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   
   # Install FFmpeg development headers
   # Ubuntu/Debian:
   sudo apt-get install libavformat-dev libavfilter-dev libavcodec-dev libavutil-dev
   
   # macOS:
   brew install ffmpeg
   
   # Arch Linux:
   sudo pacman -S ffmpeg
   ```

2. **Clone and Build**
   ```bash
   git clone https://github.com/animafps/apio.git
   cd apio
   cargo build
   ```

3. **Run Tests**
   ```bash
   cargo test
   ```

## How to Contribute

### Priority Areas (MVP Focus)

We're currently focused on building the MVP and would especially appreciate help with:

1. **Core Filter Implementations**
   - Basic video filters (scale, blur, brightness, contrast)
   - Audio filters (volume, fade, mix)
   - Filter parameter validation

2. **JSON Schema & Validation**
   - Robust JSON filter configuration parsing
   - Error handling and validation
   - Configuration file examples

3. **Performance Optimizations**
   - Memory pool improvements
   - Frame reference management
   - Benchmark implementations

4. **Documentation & Examples**
   - Filter usage examples
   - Performance comparison scripts
   - API documentation

### Future Contributions Welcome

While not immediate priorities, we also welcome early work on:
- Lua scripting integration
- VapourSynth plugin bridge
- Multi-threading support
- Encoding pipeline integration

## Contribution Process

### 1. Find or Create an Issue

- Check existing [issues](https://github.com/animafps/apio/issues) for something you'd like to work on
- For new features or bugs, create an issue first to discuss the approach
- Comment on an issue to let others know you're working on it

### 2. Fork and Branch

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/animafps/apio.git
cd apio
git checkout -b feature/your-feature-name
```

### 3. Development Guidelines

#### Code Style

- Follow standard Rust conventions (`cargo fmt`)
- Run `cargo clippy` and address warnings
- Write clear, self-documenting code with appropriate comments
- Use meaningful variable and function names

#### Testing

- Add unit tests for new functionality
- Include integration tests for filter implementations
- Ensure all tests pass before submitting: `cargo test`
- Add performance benchmarks for new filters when applicable

#### Documentation

- Document public APIs with rustdoc comments
- Update README.md if adding user-facing features
- Include usage examples for new filters

### 4. Commit Guidelines

Write clear, descriptive commit messages:

```
feat: add gaussian blur filter implementation

- Implements basic gaussian blur using FFmpeg's gblur filter
- Adds JSON configuration support for radius parameter
- Includes unit tests and usage example
```

Use conventional commit prefixes:
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test additions/modifications
- `refactor:` - Code refactoring
- `perf:` - Performance improvements

### 5. Submit Pull Request

1. Push your branch to your fork
2. Create a pull request against the main branch
3. Fill out the pull request template
4. Wait for review and address feedback

## Pull Request Template

When creating a pull request, please include:

```markdown
## Description
Brief description of changes made.

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Performance improvement
- [ ] Documentation update
- [ ] Refactoring

## Testing
- [ ] All existing tests pass
- [ ] New tests added for new functionality
- [ ] Manual testing completed

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Documentation updated if needed
- [ ] No breaking changes (or clearly documented)
```

## Performance Considerations

When contributing, keep in mind:

- **Memory Management**: Minimize allocations, reuse buffers when possible
- **FFmpeg Integration**: Use FFmpeg's native structures (AVFrame) efficiently
- **Error Handling**: Provide clear error messages and proper cleanup
- **Thread Safety**: Consider future multi-threading requirements

## Benchmarking

For performance-critical changes, include benchmarks:

```rust
#[cfg(test)]
mod benches {
    use super::*;
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    
    fn benchmark_my_filter(c: &mut Criterion) {
        c.bench_function("my_filter", |b| {
            b.iter(|| {
                // Benchmark code
            });
        });
    }
    
    criterion_group!(benches, benchmark_my_filter);
    criterion_main!(benches);
}
```

## Communication

- **GitHub Issues**: For bug reports, feature requests, and discussions
- **Pull Request Comments**: For code review discussions
- **Discord/Slack**: [Link if you have a community chat]

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please:

- Be respectful and constructive in all interactions
- Welcome newcomers and help them get started
- Focus on what is best for the community
- Show empathy towards other community members

## Questions?

If you have questions about contributing:

1. Check existing documentation and issues
2. Create a new issue with the "question" label
3. Reach out to maintainers for guidance

Thank you for helping make APIO better!
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-01-13

### Added
- Initial release of B58UUID CLI as `b58uuid-cli` package
- `encode` command to convert UUID to B58UUID
- `decode` command to convert B58UUID to UUID
- `generate` command to create random B58UUIDs
- `validate` command to check UUID/B58UUID format
- Support for reading from stdin
- Support for batch processing from files
- Colored output for better readability
- Cross-platform support (Windows, macOS, Linux)
- Optimized binary size (~475KB uncompressed, ~238KB compressed)
- Comprehensive documentation and examples

### Features
- Fast encoding/decoding (~1M ops/sec)
- Low memory footprint (<5MB)
- Zero runtime dependencies
- Pipeline-friendly design
- Alias commands (enc, dec, gen, val)
- File input support with `--file` flag
- Multiple UUID generation with `--count` flag
- Option to disable colors with `--no-color`

### Fixed
- Fixed release workflow artifact upload/download
- Fixed Homebrew formula auto-update with template approach
- Fixed RPM package build path
- Fixed workflow to handle empty commits gracefully
- Added asset verification with retry logic

### Notes
- Package name is `b58uuid-cli` to avoid conflict with `b58uuid` library
- Binary name remains `b58uuid` for convenience
- Previous version 1.0.1 published under wrong package name was yanked

[1.0.0]: https://github.com/b58uuid/b58uuid-cli/releases/tag/v1.0.0

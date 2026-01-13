# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.1] - 2026-01-13

### Fixed
- Fixed release workflow artifact upload/download to ensure binaries are not empty
- Fixed Homebrew formula auto-update to use template instead of sed
- Fixed RPM package build path
- Fixed workflow to skip commit when no changes exist

### Changed
- Changed package name from `b58uuid-cli` to `b58uuid` for simpler installation
- Improved release workflow with asset verification and retry logic
- Added 30-second wait and verification before calculating checksums

## [1.0.0] - 2026-01-12

### Added
- Initial release of B58UUID CLI
- `encode` command to convert UUID to B58UUID
- `decode` command to convert B58UUID to UUID
- `generate` command to create random B58UUIDs
- `validate` command to check UUID/B58UUID format
- Support for reading from stdin
- Support for batch processing from files
- Colored output for better readability
- Cross-platform support (Windows, macOS, Linux)
- Optimized binary size (~1MB)
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

[1.0.1]: https://github.com/b58uuid/b58uuid-cli/releases/tag/v1.0.1
[1.0.0]: https://github.com/b58uuid/b58uuid-cli/releases/tag/v1.0.0

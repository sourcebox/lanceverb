# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `Reverb::process` function to process a block of samples.
- `Reverb::process_add` function to process a block of samples with mixing.

### Changed

- Made crate `no_std` for use in embedded context.
- Replaced `Reverb::construct` by `Default` implementation.

### Removed

- `dsp-chain` as default feature.

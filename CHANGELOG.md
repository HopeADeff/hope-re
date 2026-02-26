# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [2.0.3] - 2026-02-26

### Bug Fixes

- Fix publish.yml workflow and styling corrections
- Update npm packages to latest versions ([#50](https://github.com/HopeArtOrg/hope-re/pull/50))

### Changed

- Bump version from 2.0.2-alpha to 2.0.3 stable release across package.json, Cargo.toml, and tauri.conf.json

## [2.0.2-alpha] - 2026-02-26

### Features

- UI overhaul with new file drop zone component, glaze style select, and nightshade target select ([#47](https://github.com/HopeArtOrg/hope-re/pull/47))
- ONNX model export notebook (`5_export_onnx.ipynb`) and runtime integration ([#37](https://github.com/HopeArtOrg/hope-re/pull/37))
- ONNX YAML uploading scripts for CI/CD pipeline ([#39](https://github.com/HopeArtOrg/hope-re/pull/39))

### Bug Fixes

- Publish script corrections and version bump ([#48](https://github.com/HopeArtOrg/hope-re/pull/48))
- Build errors on macOS and Linux platforms ([#45](https://github.com/HopeArtOrg/hope-re/pull/45))
- Tauri build script version mismatch ([#44](https://github.com/HopeArtOrg/hope-re/pull/44))
- Publishing branch error in CI workflow ([#43](https://github.com/HopeArtOrg/hope-re/pull/43))
- Publishing scripts for release pipeline ([#42](https://github.com/HopeArtOrg/hope-re/pull/42))
- Upload scripts for artifact distribution ([#40](https://github.com/HopeArtOrg/hope-re/pull/40))

### Changed

- Updated theme toggle, system info dialog, and base image placeholder UI ([#47](https://github.com/HopeArtOrg/hope-re/pull/47))
- Reworked app.css color scheme with oklch values ([#47](https://github.com/HopeArtOrg/hope-re/pull/47))
- Removed input-prompt, target-description-input, and target-style-select components in favor of algorithm-specific selects ([#47](https://github.com/HopeArtOrg/hope-re/pull/47))
- Updated dependency versions (SvelteKit, Svelte, ESLint, bits-ui, tailwindcss) ([#42](https://github.com/HopeArtOrg/hope-re/pull/42))

## [2.0.1-alpha] - 2025-12-24

### Features

- Main application UI with image placeholders, fullscreen dialog, adjustments menu, protection progress panel, algorithm/style/quality selectors, and intensity slider ([#16](https://github.com/HopeArtOrg/hope-re/pull/16))
- Application header with theme toggle and system info dialog ([#14](https://github.com/HopeArtOrg/hope-re/pull/14))
- Custom window titlebar for desktop platforms ([#9](https://github.com/HopeArtOrg/hope-re/pull/9))
- Tailwind CSS v4 and shadcn-svelte component library integration ([#8](https://github.com/HopeArtOrg/hope-re/pull/8))
- Tauri v2 desktop app shell with Rust backend ([#7](https://github.com/HopeArtOrg/hope-re/pull/7))
- ESLint, Husky, and lint-staged for code quality and pre-commit hooks ([#5](https://github.com/HopeArtOrg/hope-re/pull/5))
- GitHub contribution rules and MIT LICENSE ([#6](https://github.com/HopeArtOrg/hope-re/pull/6))
- Cache and state management with TanStack Svelte Query, replacing local Svelte stores ([#23](https://github.com/HopeArtOrg/hope-re/pull/23))
- ML notebook: initial Colab notebook for model development ([#25](https://github.com/HopeArtOrg/hope-re/pull/25))
- ML notebook: CLIP-to-JAX model conversion ([#27](https://github.com/HopeArtOrg/hope-re/pull/27))
- ML notebook: noise algorithm implementation ([#29](https://github.com/HopeArtOrg/hope-re/pull/29))
- ML notebook: Glaze protection algorithm ([#31](https://github.com/HopeArtOrg/hope-re/pull/31))
- ML notebook: Nightshade data poisoning algorithm ([#34](https://github.com/HopeArtOrg/hope-re/pull/34))

### Bug Fixes

- Custom window titlebar rendering on non-Windows platforms ([#12](https://github.com/HopeArtOrg/hope-re/pull/12))
- Intensity slider UI and types integrity ([#19](https://github.com/HopeArtOrg/hope-re/pull/19))
- Unnecessary comments and formatting in +page.svelte
- Naming inconsistencies in project configuration ([#32](https://github.com/HopeArtOrg/hope-re/pull/32))

### Refactoring

- Separate types into dedicated files and fix slider UI color ([#18](https://github.com/HopeArtOrg/hope-re/pull/18))
- Clearer type definitions across components ([#21](https://github.com/HopeArtOrg/hope-re/pull/21))

### Security

- Update SvelteKit and Svelte packages to avoid CVE from older versions ([#20](https://github.com/HopeArtOrg/hope-re/pull/20))

[2.0.3]: https://github.com/HopeArtOrg/hope-re/compare/v2.0.2-alpha...v2.0.3
[2.0.2-alpha]: https://github.com/HopeArtOrg/hope-re/compare/v2.0.1-alpha...v2.0.2-alpha
[2.0.1-alpha]: https://github.com/HopeArtOrg/hope-re/commits/v2.0.1-alpha

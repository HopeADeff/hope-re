# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [2.0.42] - 2026-02-28

### Bug Fixes

- Drop `x86_64-apple-darwin` (Intel Mac) build target since `ort-sys@2.0.0-rc.11` no longer provides prebuilt ONNX Runtime binaries for that platform

## [2.0.41] - 2026-02-28

### Bug Fixes

- Fix CoreML `with_subgraphs()` call to pass explicit `bool` argument required by ort v2.0.0-rc.11 API, fixing macOS and iOS builds
- Replace `native-tls` with `rustls` across all TLS-dependent crates to fix Android cross-compilation failing on missing OpenSSL
- Fix Android build failing with `ort-sys` unable to find prebuilt ONNX Runtime binaries for `x86_64-linux-android` with CUDA feature set
- Fix Android build failing for x86/x86_64 emulator targets where no ONNX Runtime prebuilt binaries exist

### Changed

- Switch `reqwest` from default features to explicit feature set with `rustls-tls` instead of `default-tls`
- Switch `ort` TLS feature from `tls-native` to `tls-rustls` for pure-Rust TLS support
- Split `ort` execution provider features into platform-conditional dependencies (CUDA + DirectML on Windows, CUDA + XNNPACK on Linux, CoreML on macOS/iOS, XNNPACK on Android)
- Restrict Android build targets to `aarch64` (ARM64) only, since ONNX Runtime has no prebuilt binaries for x86/x86_64 Android

## [2.0.4] - 2026-02-27

### Features

- Wire up real ONNX model inference for image protection using SPSA gradient estimation and PGD adversarial perturbation
- Process images via overlapping 224x224 tiles with feathered blending for seamless full-resolution output
- Pass style_index and target_index tensors to Glaze and Nightshade models respectively
- Map render_quality to PGD iteration count and intensity to perturbation epsilon for user-controllable protection strength
- Graceful fallback to simple noise when ONNX models are unavailable or fail to load
- Resolve bundled ONNX model paths via Tauri AppHandle resource directory
- Add `useProtectImage()` TanStack Svelte Query mutation for image protection invocations
- Add `buildProtectionSettings()` helper to construct protection settings from UI state
- Add `useImage()` composable for image upload (FileReader), download (Tauri dialog + fs), and fullscreen state
- Add `useProtection()` composable for protection settings state, progress simulation, and mutation execution
- Add iOS and Android GPU detection in system_info module
- Add `tauri-plugin-dialog` and `tauri-plugin-fs` plugins with Tauri capability permissions for save dialog and file writing
- Register `protect_image` Tauri command in invoke handler
- Add platform-specific ONNX Runtime execution providers (ort crate):
  - Windows x64: CUDA + DirectML
  - macOS x64/ARM64: CoreML
  - iOS: CoreML
  - Linux x64/ARM64: CUDA + XNNPACK
  - Android: XNNPACK
- Add automatic ONNX model download on first launch from GitHub Releases with streaming progress
- Add `check_models_status` and `download_model` Tauri commands for model lifecycle management
- Add `ResourceDownloadGuard` dialog component that blocks app usage until all models are downloaded
- Add `useModelDownload()` composable for sequential model download orchestration with Tauri event listening
- Add `useModelsStatus()` and `useDownloadModel()` TanStack Query hooks for model download state

### Bug Fixes

- Fix `protect_image` command not performing real image processing (was using placeholder seeded noise instead of ONNX model inference)
- Fix duplicate `build_execution_providers_internal()` dead code in protection.rs by reusing `build_execution_providers()` from capabilities.rs
- Fix glaze style mapping to match frontend values (abstract, impressionist, cubist, sketch, watercolor)
- Fix nightshade target mapping to match frontend values (dog, cat, car, landscape, person, building, food, abstract)
- Fix missing iOS and Android branches in gpu.rs `detect_gpu()` cfg dispatch
- Fix badge border-radius from rounded-full to rounded-lg for consistent Zen aesthetic
- Fix image placeholder icon containers border-radius (rounded-2xl to rounded-lg) and spacing
- Filter inference capabilities to only return execution providers where `is_available()` is true
- Remove `available` and `registered` fields from `ExecutionProviderInfo` since all listed providers are available

### Changed

- Refactor +page.svelte from 333 lines to 166 lines by extracting logic into composable files
- Move image upload, download, and fullscreen logic to `use-image.svelte.ts` composable
- Move protection settings state, progress simulation, and execution to `use-protection.svelte.ts` composable
- Convert `protectImage()` from direct async invoke to TanStack `createMutation` wrapper
- Update `protect_image` Tauri command signature to accept `AppHandle` for resource path resolution
- Update primary font from Montserrat to Be Vietnam Pro for cleaner Zen-like aesthetic
- Add JetBrains Mono Variable font for numeric displays (intensity, output quality, progress)
- Update foreground color to charcoal black (#2D2D2D / oklch(0.18 0 95)) for light mode with dark mode responsiveness
- Derive `isProcessing` from `mutation.isPending` and `resultImage` from `mutation.data` instead of duplicating in `$state`
- Remove ONNX model bundling from `tauri.conf.json` (models now downloaded at runtime)
- Refactor `protection.rs` (697 lines) into `protection/` module directory with focused files: types, model loading, preprocessing, algorithms, SPSA optimization, tiling, and encoding

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

[2.0.41]: https://github.com/HopeArtOrg/hope-re/compare/v2.0.4...v2.0.41
[2.0.4]: https://github.com/HopeArtOrg/hope-re/compare/v2.0.3...v2.0.4
[2.0.3]: https://github.com/HopeArtOrg/hope-re/compare/v2.0.2-alpha...v2.0.3
[2.0.2-alpha]: https://github.com/HopeArtOrg/hope-re/compare/v2.0.1-alpha...v2.0.2-alpha
[2.0.1-alpha]: https://github.com/HopeArtOrg/hope-re/commits/v2.0.1-alpha

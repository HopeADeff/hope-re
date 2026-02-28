# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [2.0.77] - 2026-02-28

### Performance

- Pre-allocate SPSA buffers (`direction`, `plus_data`, `minus_data`, `grad_estimate`) once outside the iteration loop instead of allocating 150,528-element vectors every iteration
- Add momentum (beta=0.9) to SPSA gradient descent for faster convergence with fewer iterations, replacing raw sign-based PGD
- Eliminate f32-to-u8-to-f32 roundtrip in tile blending by reading protected tile values directly from the Array4 instead of converting through `tile_to_pixels` intermediate buffer
- Skip bilinear interpolation in tile blending when tile dimensions match TILE_SIZE (224x224), avoiding unnecessary per-pixel math for full-size tiles
- Simplify SPSA edge weight expansion from triple-expand `[ew, ew, ew]` + flatten pattern to direct per-channel assignment, eliminating intermediate `Vec<[f32; 3]>` allocation
- Extract shared `create_image_tensor()` helper in algorithms module to deduplicate tensor creation across noise, glaze, and nightshade model runners
- Pre-allocate base64 encoding output buffer with estimated capacity based on pixel count and quality setting
- Increase `alpha_multiplier` from 2.5 to 3.5 (noise, nightshade) and 2.0 to 3.0 (glaze) for stronger per-iteration perturbation impact
- Simplify SPSA gradient computation from `diff / (2.0 * ck * direction[i])` to `diff_over_2ck * direction[i]` by factoring out the constant divisor

### Features

- Add manual update check button to header alongside system info and theme toggle
- Show toast notification when manually checking for updates and already on latest version

### Changed

- Redesign update dialog to match Zen aesthetic with `ArrowUpCircleIcon`, version subtitle, "What's new" label, monospace progress percentage, and cleaner button labels
- Convert `useUpdater()` composable from per-instance state to module-level singleton so header button and dialog share update lifecycle state
- Add `dialogOpen` and `openDialog()` to updater composable for manual dialog trigger support
- Move `seeded_rand` function from `spsa.rs` to `encoding.rs` where it is actually used, removing dead export from SPSA module
- Inline `load_session_cpu_only` into `load_model` in model.rs, removing unnecessary private function indirection
- Remove unused `tile_to_pixels` function and `Rgba` import from preprocessing module
- Group tile blend parameters into `TileRegion` struct to satisfy clippy `too_many_arguments` lint

## [2.0.76] - 2026-02-28

### Fixed

- Fix `STATUS_ACCESS_VIOLATION` crash during ONNX model inference caused by DirectML execution provider failing to handle jax2onnx-exported ops, segfaulting on partial node assignment
- Fix ONNX input name mismatch where jax2onnx auto-generated names (`in_0`, `in_1`) did not match hardcoded names (`input`, `style_index`, `target_index`) by switching to positional `ort::inputs!` macro arguments
- Fix `TensorRef` (borrowed) and `Tensor` (owned) mixed ownership in `ort::inputs!` macro causing undefined memory access with GPU execution providers
- Fix ndarray version mismatch between project (`ndarray 0.16`) and `ort` crate (`ndarray 0.17`) by constructing tensors via `(shape, Box<[T]>)` tuple form instead of passing `Array4` directly
- Fix nightshade algorithm stuck at 0% progress due to CPU inference being ~100x slower than GPU, making SPSA loop take 10-40 minutes per tile before any progress event emitted

### Changed

- Load ONNX models with CPU-only execution provider instead of DirectML/CUDA to prevent segfaults from unsupported GPU ops
- Reduce `SPSA_DIRECTIONS_PER_ITER` from 16 to 4 for feasible CPU inference times while maintaining gradient estimation quality
- Reduce noise `max_iterations` from 250 to 50, glaze from 350 to 60, nightshade from 500 to 75 to target ~1-2 minutes per tile on CPU
- Remove unused `array4_to_tensor_ref` function and `TensorRef` import from preprocessing module
- Add `rename_model_inputs()` function to ONNX export notebook to rename jax2onnx auto-generated input names to proper names for future model exports

## [2.0.75] - 2026-02-28

### Fixed

- Refactored `spsa_pgd_on_tile` function to reduce parameters from 9 to 7 by grouping progress-related parameters into a new `TileProgress` struct
- Fixed incorrect newline styles in Rust source files using `cargo fmt`

## [2.0.7] - 2026-02-28

### Features

- Add Tauri updater plugin for automatic desktop app updates with signed artifacts
- Add update dialog that checks for new versions on launch with download progress and one-click install
- Add `useUpdater()` composable for update check, download, install, and dismiss lifecycle
- Add `tauri-plugin-process` for app restart after update installation

### Performance

- Add perceptual loss term to SPSA gradient estimation matching Python `combined_loss()` pattern with `mean(diff^2 * (1.5 - edge_weight)) * perceptual_weight * 100` scaling
- Add edge-weighted gradient application before sign operation to concentrate perturbations in textured areas where they are less visible to humans
- Replace LCG-based `seeded_rand` with Xoshiro128++ PRNG for statistically independent Rademacher random directions in SPSA
- Add bilinear interpolation in tile-to-image blending replacing nearest-neighbor sampling to eliminate blocky artifacts on edge tiles
- Increase noise epsilon from 0.06 to 0.08 and iterations from 200 to 250 for stronger AI disruption
- Increase glaze epsilon from 0.035 to 0.05 and iterations from 300 to 350 for more effective style cloaking
- Increase nightshade epsilon from 0.03 to 0.045 for stronger data poisoning perturbations
- Fix noise and nightshade alpha_multiplier from 2.0 to 2.5 matching actual Python training code

### Bug Fixes

- Fix `compute_edge_weight_map()` never being called despite existing in preprocessing, leaving edge-aware perturbation weighting completely inactive
- Fix ONNX inference errors silently treated as `0.0` loss corrupting gradient estimates; add consecutive failure counter with 5-failure abort threshold
- Fix gradient averaging dividing by total directions instead of only successful directions, under-weighting gradients when some ONNX inferences fail
- Fix GELU activation mismatch in ONNX export notebook using `jax.nn.gelu(x, approximate=True)` instead of `nn.gelu(x)` matching training notebooks
- Fix ONNX models not bundled in desktop production builds because `resources` array in `tauri.conf.json` was empty, causing silent fallback to basic random noise instead of CLIP-guided adversarial perturbation
- Fix SPSA optimization producing weak perturbations due to low default intensity (20/100, epsilon=0.024) being 40% of training baseline (epsilon=0.06); raise default to 50/100 to match training
- Fix SPSA optimization under-converging due to low default render quality (50%) halving iteration count; raise default to 75%

### Changed

- Rotate updater signing keypair and update public key in Tauri config to fix incorrect private key password error during builds
- Add `TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` to publish workflow for signing updater artifacts
- Add `updaterJsonPreferNsis` to publish workflow for Windows NSIS installer preference in `latest.json`
- Add `updater:default` and `process:allow-restart` permissions to Tauri capabilities
- Add `createUpdaterArtifacts` to bundle config and updater plugin config with public key and GitHub Releases endpoint
- Add `perceptual_weight` field to `AlgorithmParams` struct with per-algorithm defaults (noise: 0.5, glaze: 1.0, nightshade: 1.5)
- Pass edge weight map from `tiling.rs` through to `spsa_pgd_on_tile()` for per-tile edge-aware optimization
- Bundle ONNX models in desktop builds via `"resources": ["../src-models/models/*.onnx"]` in `tauri.conf.json`
- Raise default intensity slider from 20 to 50 and default render quality from 50 to 75 in `use-protection.svelte.ts`
- Add prominent amber warning banner on the main page when protection completes with fallback (model not loaded)
- Expose `modelUsed` and `resultMessage` from `useProtection()` composable for downstream UI consumption

## [2.0.44] - 2026-02-28

### Bug Fixes

- Fix CUDA inference incorrectly reported as available on systems without NVIDIA GPU by adding `has_nvidia_gpu()` hardware check before `CUDA::default().is_available()`
- Fix `render_quality=0` ("Faster" preset) producing unchanged images because `render_factor` was multiplied into intensity, making effective perturbation zero
- Fix intensity and render quality settings conflated into a single multiplier instead of independently controlling perturbation strength and iteration count
- Fix `render_quality` clamp range from `1..100` to `0..100` to accept the "Faster" preset value of 0
- Fix fallback protection silently returning success with no log output in release builds by removing `cfg!(debug_assertions)` guard from `log::warn!` calls
- Fix Android build failing for x86/x86_64 emulator targets by moving `ort` dependency from global `[dependencies]` to target-specific sections that exclude unsupported architectures
- Fix protection progress stuck at 90% because frontend used fake `setInterval` simulation instead of real backend progress events
- Fix all buttons rendering as circles/pills by changing `rounded-full` to `rounded-lg` in the `buttonVariants` base class
- Fix rendered image overlay buttons (download/fullscreen) using explicit `rounded-full` override instead of `rounded-lg`
- Fix dialog close button using `rounded-xs` instead of `rounded-lg` for consistent corner radius
- Fix stale protected image from previous render visible when starting a new protection run by calling `mutation.reset()` before each new run

### Changed

- Rewrite `apply_fallback_noise()` from single-pass random noise to multi-iteration algorithm with varying block scales, decreasing step sizes, and accumulated perturbations clamped to epsilon bounds
- Add `model_used` field to `ProtectionResult` so frontend can distinguish real ONNX model protection from fallback noise
- Show warning toast "Image protected with basic fallback. Download AI models for stronger protection." when fallback is used instead of misleading success message
- Add `onnx_stubs` module with stub Tauri commands for unsupported Android architectures (x86, x86_64) to allow compilation without ONNX Runtime
- Gate `onnx_integration` module behind `cfg` to exclude unsupported Android targets
- Replace fake frontend progress simulation with real `protection-progress` Tauri events emitted from Rust backend during tile processing and SPSA iterations
- Add `ProtectionProgress` struct with stage, tile/iteration counts, and percentage for granular progress reporting
- Pass `AppHandle` through `apply_model_protection` and `spsa_pgd_on_tile` to enable event emission
- Update frontend `useProtection()` to use `listen()` for `protection-progress` events instead of `setInterval`
- Update render quality time estimates to realistic ranges based on actual iteration counts (e.g., "~1-2 mins" for Faster, "~10-30 mins" for Default)
- Remove fake "Estimated time remaining" formula from progress component
- Update rendered image overlay button backgrounds from `bg-background/60` to `bg-muted/60` for subtler Zen aesthetic
- Rename "Cancel" button to "Clear" with `RotateCcwIcon` when not processing; keep "Cancel" with `XIcon` during active protection

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

[2.0.77]: https://github.com/HopeArtOrg/hope-re/compare/v2.0.76...v2.0.77
[2.0.76]: https://github.com/HopeArtOrg/hope-re/compare/v2.0.75...v2.0.76
[2.0.75]: https://github.com/HopeArtOrg/hope-re/compare/v2.0.7...v2.0.75
[2.0.7]: https://github.com/HopeArtOrg/hope-re/compare/v2.0.44...v2.0.7
[2.0.44]: https://github.com/HopeArtOrg/hope-re/compare/v2.0.42...v2.0.44
[2.0.42]: https://github.com/HopeArtOrg/hope-re/compare/v2.0.41...v2.0.42
[2.0.41]: https://github.com/HopeArtOrg/hope-re/compare/v2.0.4...v2.0.41
[2.0.4]: https://github.com/HopeArtOrg/hope-re/compare/v2.0.3...v2.0.4
[2.0.3]: https://github.com/HopeArtOrg/hope-re/compare/v2.0.2-alpha...v2.0.3
[2.0.2-alpha]: https://github.com/HopeArtOrg/hope-re/compare/v2.0.1-alpha...v2.0.2-alpha
[2.0.1-alpha]: https://github.com/HopeArtOrg/hope-re/commits/v2.0.1-alpha

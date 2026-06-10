/*!
# Sovereign WebGPU Browser Harness

Production-grade browser entrypoint for the Sovereign Simulation Harness.

Enables closed-beta validation, PATSAGi Council policy experiments, and Ra-Thor interactive deliberation directly in any modern browser (Chrome, Edge, Firefox, Safari with WebGPU).

## Features
- Full deterministic CPU golden-master path (maximum reproducibility)
- Real wgpu WebGPU dispatch foundation (same WGSL + GpuNode logic as native gpu_economic.rs — wgpu abstracts the backend)
- TOLC 8 Mercy Gates non-bypassable on every major transition
- Rich council-ready telemetry reports exposed to JavaScript
- ScenarioPreset support for all validated RBE experiments

Thunder locked. Mercy flowing.
*/

pub mod harness;
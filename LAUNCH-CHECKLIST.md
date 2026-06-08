# Powrush-MMO Global Professional Release Launch Checklist
## v17.0 — SIMD Query Integrated into HierarchicalGrid

**Status**: SIMD acceleration now active in production path.

- `query_radius` uses f32x8 SIMD for chunks of 8 entities
- Automatic scalar fallback for small datasets
- Full SoA (x/y/z) layout enables clean vectorization

**Thunder locked in.** ⚡❤️🔥
# SCU Engine — Scalable Computational Urbanism

SCU Engine explores whether prototype-instancing and spatial bucketing can
reduce BIM processing costs for large building-scale models. The project
focuses on benchmark-driven evaluation, synthetic datasets, and reproducible
performance analysis.

## Architecture

|         Crate            |           Purpose             | Status |
|--------------------------|-------------------------------|--------|
|       `scu-schema`       |   Shared `#[repr(C)]` types   |   S1   |
|        `scu-gen`         |  Synthetic dataset generator  |   S2   |
|        `scu-core`        |    Engine + benchmark suite   |   S3   |
|     `scu-harvester`      |         IFC ingestion         |  S3/S4 |
|        `scu-cli`         |     CLI runner (`run.py`)     |   S3   |

## Current Development

- Core architecture planned
- Rust implementation in progress
- Benchmark framework under design

## Milestones

- [ ] M1: `scu-schema` compiles
- [ ] GEN: `scu-gen` with P/N + hotspot controls
- [ ] M4: Benchmark suite (Chart 2 primary)
- [ ] P2a: Paper #2 submitted

## Benchmarks (planned)

- Chart 2: P/N ratio vs speedup (primary)
- Charts 1, 5, 6: Scaling ablation (Naive / Instancing / SCU Full)
- Chart 3: Real IFC categorical
- Chart 4: Bucket size sensitivity

## Build

```bash
cargo check --workspace
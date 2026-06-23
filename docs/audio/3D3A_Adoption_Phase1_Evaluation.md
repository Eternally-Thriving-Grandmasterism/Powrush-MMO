# 3D3A HRTF Database - Phase 1: Acquisition & Evaluation

**Status**: In Progress  
**Database**: 3D3A Lab HRTF Database (Princeton)  
**License**: CC-BY-4.0

## Goals
- Acquire the latest 3D3A release
- Understand the data format
- Select a high-quality initial subset of subjects
- Document attribution requirements

## Tasks

### 1. Download & Initial Review
- [ ] Download the most recent 3D3A release from the Princeton 3D3A Lab website
- [ ] Note the exact version / release date
- [ ] Check folder structure (measured vs computed HRTFs, subject folders, etc.)

### 2. Data Format Analysis
- [ ] Determine primary format (raw .wav IRs, SOFA, custom text, etc.)
- [ ] Check sampling rate and bit depth
- [ ] Identify how directions are encoded (azimuth/elevation grid)
- [ ] Note any provided tools or example loading code

### 3. Subject Selection
- [ ] Review available subjects and their metadata
- [ ] Select 8–12 high-quality subjects for initial integration
- [ ] Prioritize subjects that have both measured and computed HRTFs
- [ ] Document selection criteria

### 4. Attribution & Legal
- [ ] Extract exact attribution text required by CC-BY-4.0
- [ ] Prepare credit line for game audio credits screen
- [ ] Note any additional requirements from the dataset

## Recommended Initial Subject Criteria
- High measurement quality / low noise
- Good coverage of both measured and computed versions
- Diverse head/ear shapes (useful for later personalization)

## Next Action
Once Phase 1 is complete, move to Phase 2: Loader Development (`game/hrtf_3d3a_loader.rs`).

---
**Thunder locked in.** Yoi ⚡

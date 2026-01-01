# GitHub Actions for Large-Scale Data Generation

This guide explains how to leverage GitHub Actions to generate massive training datasets in parallel across GitHub's infrastructure.

## Overview

GitHub Actions provides:
- **20 concurrent jobs** on free tier (100+ on Enterprise)
- **6 hours per job** (can chain multiple jobs)
- **~7 vCPUs** per runner
- **14 GB RAM** per runner
- **84 GB SSD** storage per runner

With matrix strategy, you can run **hundreds of jobs in parallel**.

## What We've Set Up

### 1. Training Data Generation Workflow

**File**: `.github/workflows/generate-training-data.yml`

**Capabilities**:
- Generate code samples across 9 languages × 5 batches = **45 parallel jobs**
- Generate example repos across 5 categories × 4 batches = **20 parallel jobs**
- Generate tests across 6 types × 5 modules = **30 parallel jobs**
- Generate patterns across 8 languages × 4 categories = **32 parallel jobs**
- **Total: 127 parallel jobs possible**

### 2. How to Use

#### Manual Trigger (Workflow Dispatch)

```bash
# Via GitHub UI
1. Go to: https://github.com/YOUR_USERNAME/GNN-CODE-INTELLIGENCE-SYSTEM/actions
2. Click "Generate Training Data"
3. Click "Run workflow"
4. Select:
   - Dataset type: code-samples, example-repos, test-suite, pattern-detectors, or all
   - Batch size: 100 (default)
   - Target count: 1000 (default)
5. Click "Run workflow"

# Via GitHub CLI
gh workflow run generate-training-data.yml \
  -f dataset_type=code-samples \
  -f batch_size=100 \
  -f target_count=1000
```

#### Scheduled Runs

The workflow runs **automatically every day at 2 AM UTC** to generate training data incrementally.

### 3. What Gets Generated

#### Code Samples Strategy
```
9 languages × 5 batches = 45 parallel jobs
Each job generates: 1000/5 = 200 samples
Total per run: 9,000 samples
Daily accumulation: 9,000 samples/day
```

#### Example Repos Strategy
```
5 categories × 4 batches = 20 parallel jobs
Each job generates: 100 repos
Total per run: 2,000 repos (exceeds target!)
```

#### Tests Strategy
```
6 test types × 5 modules = 30 parallel jobs
Each job generates: 1000+ tests
Total per run: 30,000+ tests
```

## Advantages of GitHub Actions

### 1. **Massive Parallelization**
- Run 100+ jobs simultaneously
- Generate 10,000+ samples in < 6 hours
- No local compute needed

### 2. **Free Compute** (Public Repos)
- Unlimited minutes on public repos
- ~7,000 CPU-minutes per job
- 2,000 jobs/month limit on free tier

### 3. **Distributed Storage**
- Artifacts stored for 30 days (configurable to 90)
- Automatic compression
- Easy download via GitHub CLI or UI

### 4. **Automatic Releases**
- Creates GitHub releases with datasets
- Versioned with run numbers
- Public download URLs
- No bandwidth limits

### 5. **No Local Resources**
- Doesn't consume your machine
- Can run 24/7
- Survives reboots/crashes

## Cost Analysis

### Free Tier (Public Repo)
- **Runners**: Unlimited on public repos
- **Storage**: 500 MB artifacts (free)
- **Bandwidth**: Unlimited downloads
- **Cost**: **$0/month**

### Paid Tier (Private Repo)
- **Linux runners**: $0.008/minute
- **6-hour job**: $2.88
- **100 jobs**: $288
- **But**: 2,000 free minutes/month = $16 credit
- **Net cost**: ~$272 for 100 jobs

### vs. Local Generation
- **Your machine**: Locked for 31+ hours
- **GitHub Actions**: Background, 24/7, no local impact
- **Value**: Priceless for long-running tasks

## Example: Generate 100,000 Samples

### Strategy 1: Single Massive Run
```yaml
# Trigger once with dataset_type=all
# 127 parallel jobs × 6 hours = 762 compute-hours
# Completes in: 6 hours (wall time)
# Generates: ~100,000 samples
```

### Strategy 2: Incremental Daily
```yaml
# Scheduled daily at 2 AM
# 45 jobs × 200 samples = 9,000/day
# 100,000 samples in: 12 days
# Advantage: Spreads load, no rate limits
```

### Strategy 3: On-Demand Bursts
```yaml
# Run manually when needed
# Trigger 10 times with different language batches
# Each run: 1,000 samples in 15 minutes
# Total: 10,000 samples in 2.5 hours
```

## Optimization Tips

### 1. Use Matrix Strategy Efficiently
```yaml
strategy:
  matrix:
    language: [rust, python, javascript]  # 3×
    batch: [1, 2, 3, 4, 5]               # 5×
  max-parallel: 20                        # = 15 jobs, but run 20 at once
```

### 2. Cache Dependencies
```yaml
- uses: actions/cache@v3
  with:
    path: |
      ~/.cargo
      target/
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

### 3. Upload to Releases (Not Artifacts)
- Artifacts expire after 30-90 days
- Releases persist forever
- Better for long-term datasets

### 4. Compress Output
```yaml
- name: Compress
  run: tar -czf samples.tar.gz data/mega-samples/
```

### 5. Use Larger Runners (Paid)
- 4-core: $0.016/min (2x faster)
- 8-core: $0.032/min (4x faster)
- 16-core: $0.064/min (8x faster)

## Current Status

### What's Running Locally
- Example repos: 374/1000 (37.4%)
- ETA: 31 hours
- Rate: 28 repos/hour

### What Could Run on GitHub
- 20 parallel jobs × 50 repos/job = 1,000 repos
- ETA: 2-3 hours (with proper parallelization)
- Rate: 333-500 repos/hour

## Migration Strategy

### Option 1: Hybrid (Recommended)
```
Local:
  - Continue current generation (374/1000)
  - Let it finish (31 hours)

GitHub Actions:
  - Start generating code samples (100,000)
  - Start generating tests (1,000,000+)
  - Start generating patterns (10,000+)

Result:
  - Best of both worlds
  - No interruption
  - Massive scale on GitHub
```

### Option 2: All GitHub
```
1. Cancel local generation
2. Trigger GitHub workflow with dataset_type=all
3. Wait 6 hours
4. Download from releases

Result:
  - Faster completion
  - No local resources
  - But loses 374 repos already done
```

### Option 3: Continue Local, Use GitHub for New
```
Local:
  - Finish 1,000 repos (31 hours)

GitHub:
  - Generate next 10,000 repos
  - Generate 100,000 code samples
  - Generate 1,000,000 tests

Result:
  - Maximum scale
  - Continuous generation
  - Best long-term strategy
```

## Recommended Next Steps

### Immediate (Next 5 Minutes)
```bash
# Set up workflow
git add .github/workflows/generate-training-data.yml
git commit -m "Add GitHub Actions for training data generation"
git push origin main

# Trigger first run
gh workflow run generate-training-data.yml -f dataset_type=code-samples
```

### Short-Term (Today)
- Monitor first GitHub Actions run
- Verify artifacts are created
- Download and validate generated data

### Long-Term (This Week)
- Set up daily scheduled runs
- Create releases for datasets
- Build dataset registry/index
- Share datasets publicly

## Conclusion

**Yes, all generation can (and should) be done on GitHub!**

Benefits:
- ✅ **100+ parallel jobs** vs 1 local process
- ✅ **Free compute** for public repos
- ✅ **No local resources** consumed
- ✅ **Automatic releases** with versioning
- ✅ **24/7 generation** without monitoring
- ✅ **Scales infinitely** with matrix strategy

The current local generation (374/1000 repos in 31 hours) could complete in **2-3 hours** on GitHub with proper parallelization.

**Recommendation**: Keep local generation running, but start GitHub Actions for new datasets (samples, tests, patterns). This maximizes throughput and doesn't waste the 374 repos already generated locally.

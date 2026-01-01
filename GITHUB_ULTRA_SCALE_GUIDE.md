# GitHub Actions Ultra-Scale Guide

## Maximum Free Tier Exploitation

This guide shows how to push GitHub Actions to its absolute limits while remaining 100% free.

---

## GitHub Free Tier Limits (2025)

### Hard Limits
- ✅ **Concurrent jobs**: 20 (Free tier)
- ✅ **Jobs per workflow**: 256 (matrix limit)
- ✅ **Job runtime**: 6 hours each
- ✅ **Workflow queueing**: 500 per 10 seconds
- ✅ **Minutes**: **UNLIMITED** for public repos
- ✅ **Storage**: 500 MB artifacts, 10 GB cache
- ✅ **Bandwidth**: Unlimited (via GitHub Releases)

### No Limits
- ✅ Number of workflows
- ✅ Number of workflow runs
- ✅ Total CPU-hours (for public repos)
- ✅ Release artifacts (use releases, not artifacts)

---

## Scale Levels

### Level 1: Standard (Current)
**Command**:
```bash
gh workflow run generate-training-data.yml -f dataset_type=all
```

**Specs**:
- Jobs: 127 total
  - 45 code samples (9 langs × 5 batches)
  - 20 example repos (5 cats × 4 batches)
  - 30 tests (6 types × 5 modules)
  - 32 patterns (8 langs × 4 categories)
- Runtime: ~6 hours
- Output: ~100K samples, 1K repos, 30K tests, 3K patterns
- Cost: **$0**

### Level 2: Maximum (256 Jobs)
**Command**:
```bash
gh workflow run ultra-massive-generation.yml -f scale=maximum
```

**Specs**:
- Jobs: 255 (GitHub limit: 256)
  - 200 code samples (20 langs × 10 batches)
  - 100 repos (10 cats × 10 batches)
  - 50 tests (10 types × 5 batches)
  - 60 patterns (12 langs × 5 categories)
- Runtime: ~6-8 hours (20 concurrent)
- Output: 1M samples, 10K repos, 500K tests, 60K patterns
- Compute: 1,530 CPU-hours
- Cost: **$0**

### Level 3: Ludicrous (Multi-Workflow)
**Command**:
```bash
gh workflow run ludicrous-mode-orchestrator.yml -f parallel_workflows=10
```

**Specs (10 workflows)**:
- Workflows: 10 parallel
- Jobs: 2,560 total (10 × 256)
- Runtime: ~6-8 hours (queued execution)
- Output:
  - 10M code samples
  - 100K repositories
  - 5M tests
  - 600K patterns
- Compute: 15,360 CPU-hours
- Cost: **$0**

### Level 4: PLANET-SCALE (100 Workflows)
**Command**:
```bash
gh workflow run ludicrous-mode-orchestrator.yml -f parallel_workflows=100
```

**Specs**:
- Workflows: 100 parallel
- Jobs: 25,600 total (100 × 256)
- Runtime: ~8-12 hours (massive queue)
- Output:
  - **100M code samples**
  - **1M repositories**
  - **50M tests**
  - **6M patterns**
- Compute: **153,600 CPU-hours** (17.5 CPU-years!)
- Storage: ~500 GB compressed
- Cost: **$0** (public repo)

---

## How It Works

### Matrix Strategy Exploitation
GitHub's matrix strategy creates a Cartesian product:
```yaml
strategy:
  matrix:
    language: [rust, python, js, ...]  # 20 languages
    batch: [1, 2, 3, ..., 10]          # 10 batches
  max-parallel: 20  # Free tier limit
```
Result: **200 jobs** from 2 dimensions

### Workflow Chaining
Trigger multiple workflows programmatically:
```bash
for i in {1..100}; do
  gh workflow run ultra-massive-generation.yml -f scale=maximum
done
```
Result: **25,600 jobs** (100 workflows × 256 jobs)

### Release-Based Storage
Artifacts expire after 30 days, but releases are permanent:
```yaml
- uses: softprops/action-gh-release@v1
  with:
    tag_name: ultra-v${{ github.run_number }}
    files: output/*.tar.gz
```

---

## Practical Limits

### What Limits You
1. **Concurrency**: 20 jobs at once (Free tier)
   - Solution: Jobs queue automatically
   - Impact: Longer wall time, not compute time

2. **Rate Limiting**: API requests
   - GitHub API: 5,000 req/hour (authenticated)
   - Solution: Workflow dispatch pacing (built-in)

3. **Storage**: 500 MB artifact limit
   - Solution: Use GitHub Releases (unlimited)
   - Compress: tar.gz reduces size 5-10x

4. **Job Timeout**: 6 hours max
   - Solution: Design jobs to complete in < 6 hours
   - Batch sizes: Adjust to fit time limit

### What Doesn't Limit You
- ✅ Total compute time (unlimited for public repos)
- ✅ Number of workflows
- ✅ Number of jobs (up to 256 per workflow)
- ✅ Total storage (via releases)
- ✅ Bandwidth

---

## Cost Analysis

### Free Tier (Public Repo)
```
Compute:     $0 (unlimited)
Storage:     $0 (releases)
Bandwidth:   $0 (unlimited)
Total:       $0
```

### Equivalent Commercial Cost
If you paid for 153,600 CPU-hours:
- AWS EC2 (c5.xlarge): 153,600 hours × $0.17/hour = **$26,112**
- GCP Compute (n1-standard-4): 153,600 hours × $0.19/hour = **$29,184**
- Azure (D4s_v3): 153,600 hours × $0.192/hour = **$29,491**

**GitHub Actions savings: $26,000+** for planet-scale run!

---

## Scaling Strategy

### Week 1: Test & Validate
```bash
# Start small
gh workflow run ultra-massive-generation.yml -f scale=standard

# Verify output quality
gh release download ultra-v1
tar -xzf samples-*.tar.gz
```

### Week 2: Scale to Maximum
```bash
# Single workflow, 256 jobs
gh workflow run ultra-massive-generation.yml -f scale=maximum

# Monitor progress
gh run watch
```

### Week 3: Multi-Workflow
```bash
# 10 workflows = 2,560 jobs
gh workflow run ludicrous-mode-orchestrator.yml -f parallel_workflows=10

# Check queue status
gh run list --limit 50
```

### Week 4: Planet-Scale
```bash
# 100 workflows = 25,600 jobs
gh workflow run ludicrous-mode-orchestrator.yml -f parallel_workflows=100

# Monitor across all runs
gh run list --workflow=ultra-massive-generation.yml --limit 100
```

---

## Monitoring & Management

### Check Progress
```bash
# List all workflow runs
gh run list

# Watch specific run
gh run watch <run-id>

# View logs for failed jobs
gh run view <run-id> --log-failed
```

### Download Results
```bash
# List releases
gh release list

# Download specific release
gh release download ultra-v123

# Download all (WARNING: may be hundreds of GB)
gh release list | awk '{print $3}' | xargs -I {} gh release download {}
```

### Cleanup (if needed)
```bash
# Delete old releases (free space)
gh release delete ultra-v1 --yes

# Cancel running workflows
gh run cancel <run-id>

# Cancel all queued runs
gh run list --status queued | awk '{print $7}' | xargs -I {} gh run cancel {}
```

---

## Best Practices

### 1. Start Small, Scale Gradually
- Test with `scale=standard` first
- Validate output quality
- Then scale to `maximum` and `ludicrous`

### 2. Use Releases, Not Artifacts
- Artifacts expire after 30 days
- Releases are permanent and free
- Better for long-term datasets

### 3. Compress Everything
```bash
tar -czf output.tar.gz data/
# 5-10x size reduction
```

### 4. Monitor API Rate Limits
- 5,000 requests/hour limit
- Pace workflow triggers (built into orchestrator)
- Use `sleep` between triggers if needed

### 5. Design for 6-Hour Jobs
- Each job must complete in < 6 hours
- Adjust batch sizes accordingly
- Use `timeout-minutes: 360` in workflow

---

## Theoretical Maximum

### Absolute GitHub Limits
- Workflow queue: 500 per 10 seconds
- Trigger rate: 3,000 workflows/minute
- In 1 hour: 180,000 workflows
- Jobs: 180,000 × 256 = **46,080,000 jobs**

### In Practice
- Concurrency: 20 jobs (Free tier)
- Queue depth: Thousands (no documented limit)
- Realistic max: **1,000 workflows** (256,000 jobs)
- Wall time: ~7-10 days (20 concurrent)
- Compute: **1,536,000 CPU-hours** (175 CPU-years!)
- Cost: **$0** (public repo)

---

## Current Deployment

### Active Workflows
1. ✅ `generate-training-data.yml` - Standard (127 jobs)
2. ✅ `ultra-massive-generation.yml` - Maximum (256 jobs)
3. ✅ `ludicrous-mode-orchestrator.yml` - Multi-workflow

### Running Now
- Workflow: `generate-training-data.yml`
- Run: #20634129373
- Jobs: 20 parallel (example repos)
- Status: In progress
- ETA: ~2-3 hours

### Local Generation
- Repos: 601/1000 (60.1%)
- Running in background
- Will complete in ~20 hours

---

## Next Steps

### Immediate
1. ✅ Wait for current workflow to complete
2. ✅ Validate output quality
3. ✅ Download and inspect datasets

### This Week
4. Trigger `ultra-massive-generation.yml` (256 jobs)
5. Monitor performance and output
6. Optimize batch sizes based on results

### Next Week
7. Scale to `ludicrous-mode` (10 workflows)
8. Download and merge all datasets
9. Begin GNN training on massive scale data

### Long-Term
10. Consider Enterprise tier for 500 concurrent jobs
11. Or: Deploy self-hosted runners (unlimited!)
12. Continue scaling to planet-scale datasets

---

## Summary

**You can generate virtually unlimited data on GitHub Actions for $0!**

The only real limit is concurrency (20 jobs on free tier), but:
- Jobs queue automatically
- You can trigger unlimited workflows
- Wall time increases, but total compute doesn't
- Everything is free for public repositories

**Recommendation**: Start with Level 2 (256 jobs), validate, then scale to Level 3+ (multi-workflow) for true planet-scale generation.

**Current capacity**:
- 20 concurrent jobs × 6 hours = 120 CPU-hours/day
- 256 jobs queued = 1,536 CPU-hours per workflow
- 100 workflows = 153,600 CPU-hours = **17.5 CPU-years of work!**

**All for $0** 🚀

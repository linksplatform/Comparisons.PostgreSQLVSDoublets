# Case Study: Broken Image Links in README.md (Issue #15)

## Summary

Benchmark execution resulted in broken image links in README.md. The images `bench_rust.png` and `bench_rust_log_scale.png` were not present in the `Docs/` folder, causing broken image links on GitHub.

## Timeline of Events

### Phase 1: Initial State (Before December 27, 2025)
- Benchmark images existed in the `Docs/` folder
- README.md referenced images at `Docs/bench_rust.png` and `Docs/bench_rust_log_scale.png`
- Images were being generated and committed by CI on push to main

### Phase 2: The Problematic Commit (December 27, 2025, 12:15 UTC)
- **Commit**: `b8573e9` - "Apply best practices from Comparisons.Neo4jVSDoublets"
- **Issue**: This commit **deleted** the existing benchmark images and replaced them with only a `.gitkeep` file
- **Git diff shows**:
  ```
  diff --git a/Docs/.gitkeep b/Docs/.gitkeep
  new file mode 100644
  diff --git a/Docs/bench_rust.png b/Docs/bench_rust.png
  deleted file mode 100644
  diff --git a/Docs/bench_rust_log_scale.png b/Docs/bench_rust_log_scale.png
  deleted file mode 100644
  ```

### Phase 3: Subsequent Commits (December 27, 2025)
- `3f61b63` - Initial commit with task details
- `1a6550b` - Revert "Initial commit with task details"
- `e2eeed2` - Fix CodeFactor style issues in Python output scripts
- `e99c5cd` - Make benchmark parameters configurable via environment variables
- `04547d8` - Merge pull request #14 (current main)

### Phase 4: CI Runs After the Deletion
- All subsequent CI runs showed warnings: "No files were found with the provided path: rust/bench_rust.png rust/bench_rust_log_scale.png"
- This indicates the `out.py` script was either:
  1. Not running at all (due to condition `github.event_name == 'push'` not being met for PRs), OR
  2. Running but failing silently, OR
  3. Running successfully but images not being committed

## Root Cause Analysis

### Primary Root Cause
**The commit `b8573e9` deleted the benchmark images without the CI having regenerated them first.**

When refactoring to "apply best practices from Comparisons.Neo4jVSDoublets", the commit:
1. Updated the workflow to generate and commit images on push to main
2. But also deleted the existing images from the `Docs/` folder
3. Created only a `.gitkeep` file in their place

### Secondary Root Cause
**The CI workflow only generates images on push to main/master, not on PR merges.**

Looking at the workflow condition:
```yaml
- name: Prepare benchmark results
  if: github.event_name == 'push' && (github.ref == 'refs/heads/main' || github.ref == 'refs/heads/master')
```

This means:
- PR checks: Images are NOT generated (condition not met)
- Push to main: Images ARE supposed to be generated

### Tertiary Root Cause
**The CI annotation shows "No files were found" even for push-to-main runs.**

CI run #15057894427 (push to main on May 16, 2025) shows:
> "No files were found with the provided path: rust/bench_rust.png rust/bench_rust_log_scale.png. No artifacts will be uploaded."

This suggests the `out.py` script is running but the benchmark output file (`out.txt`) may be empty or malformed when the `Prepare benchmark results` step runs.

## Evidence

### 1. Git History
```
568275a - Has images in Docs/
a67bf15 - Has images in Docs/
b8573e9 - Only .gitkeep in Docs/ (IMAGES DELETED HERE)
```

### 2. CI Run Annotations
From run #15057894427:
```
! No files were found with the provided path: rust/bench_rust.png
rust/bench_rust_log_scale.png. No artifacts will be uploaded.
```

### 3. Local Testing
The `out.py` script works correctly when given valid `out.txt` data:
- Parses 16 PSQL entries
- Parses 32 Doublets entries
- Generates both PNG images successfully

### 4. Screenshot Evidence
The issue screenshot shows broken image placeholders in README.md where benchmark charts should appear.

## Proposed Solutions

### Solution 1: Restore Images from Git History (Immediate Fix)
```bash
git checkout 568275a -- Docs/bench_rust.png Docs/bench_rust_log_scale.png
git add Docs/
git commit -m "Restore benchmark images from before deletion"
```

### Solution 2: Fix CI Workflow (Long-term Fix)
1. Ensure the `Prepare benchmark results` step runs correctly
2. Add error handling to detect when images aren't generated
3. Consider keeping images committed instead of regenerating each time

### Solution 3: Update README to Use Relative Paths (Alternative)
If images are stored in a different location, update README.md references accordingly.

## Verification Steps

1. Restore images from git history
2. Verify images appear correctly in README.md on GitHub
3. Trigger a new push to main and verify CI workflow regenerates images
4. Confirm no "No files were found" warnings in CI logs

## Lessons Learned

1. **Don't delete files until replacement mechanism is verified**: The commit deleted images assuming CI would regenerate them, but CI conditions prevented regeneration.

2. **Test workflow changes end-to-end**: The workflow refactoring should have been tested on a push to main to ensure images are actually generated.

3. **Add CI checks for required artifacts**: The workflow should fail (not just warn) if expected output files are missing.

## Related Resources

- Reference implementation: https://github.com/linksplatform/Comparisons.Neo4jVSDoublets
- Issue: https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/issues/15
- Problematic commit: b8573e91a0651496397c44c4fd3be14ae1b79bf8

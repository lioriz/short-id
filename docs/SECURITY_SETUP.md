# Security Setup for Releases

This guide shows how to configure your repository so only authorized people (admins/owners) can trigger releases.

## Quick Setup Checklist

- [ ] Configure tag protection rules
- [ ] Set up branch protection for `main`
- [ ] Configure GitHub Environment protection
- [ ] Verify permissions
- [ ] Test with a dry-run

## 1. Tag Protection Rules

**Purpose:** Prevent unauthorized users from creating version tags.

**Steps:**

1. Go to: https://github.com/lioriz/short-id/settings/tag_protection
2. Click **"New rule"**
3. **Tag name pattern:** `v*`
4. Click **"Add rule"**

This creates a basic protection rule. For more control:

### Enhanced Tag Protection (GitHub Pro/Enterprise)

If you have GitHub Pro or Enterprise:

1. Go to: https://github.com/lioriz/short-id/settings/rules
2. Click **"New ruleset"**
3. Configure:
   - **Ruleset Name:** "Version Tags"
   - **Enforcement status:** Active
   - **Target:** Tags
   - **Tag name pattern:** `v*`
4. Under **Rules:**
   - ✅ **Block creations** 
   - Add **Bypass list:** Only repository admins
5. **Create**

**Result:** Only admins can push tags starting with `v`.

## 2. Branch Protection for `main`

**Purpose:** Control who can modify version numbers in Cargo.toml.

**Steps:**

1. Go to: https://github.com/lioriz/short-id/settings/branches
2. Click **"Add rule"** or **"Add branch protection rule"**
3. **Branch name pattern:** `main`
4. Enable these settings:

### Required Settings:

- ✅ **Require a pull request before merging**
  - Require approvals: `1` (or more)
  - Dismiss stale reviews: ✅
  
- ✅ **Require status checks to pass before merging**
  - Select: `Build and Test` (your CI workflow)
  - Require branches to be up to date: ✅

- ✅ **Require conversation resolution before merging**

- ✅ **Restrict who can push to matching branches**
  - Enable and leave empty (only admins)

- ✅ **Do not allow bypassing the above settings**

- ❌ **Allow force pushes** (Disabled)
- ❌ **Allow deletions** (Disabled)

5. **Save changes**

**Result:** All changes to `main` require PR + approval + passing CI.

## 3. GitHub Environment Protection

**Purpose:** Add approval requirement before publishing to crates.io.

The workflow already uses `environment: production`. Now configure it:

**Steps:**

1. Go to: https://github.com/lioriz/short-id/settings/environments
2. If "production" doesn't exist, it will be created on first workflow run
3. Click **"production"** environment
4. Configure **Environment protection rules:**

### Recommended Settings:

- ✅ **Required reviewers**
  - Add yourself and other trusted maintainers
  - Workflow will pause and wait for approval before publishing

- ✅ **Wait timer** (optional)
  - Set to `5` minutes
  - Gives you time to cancel if needed

- ✅ **Deployment branches**
  - Select "Protected branches only"
  - Only deploys from protected branches

5. **Save protection rules**

**Result:** Workflow will pause before `cargo publish` and wait for your approval.

## 4. Secrets and Permissions

### A. Protect Secrets

Your `CARGO_REGISTRY_TOKEN` is critical. Verify it's secure:

1. Go to: https://github.com/lioriz/short-id/settings/secrets/actions
2. Verify `CARGO_REGISTRY_TOKEN` exists
3. Click **⚙️** → **Update secret** if needed

**Best practices:**
- Never share this token
- Rotate it periodically (every 6-12 months)
- Use a token with minimal scope ("publish-update" only)

### B. Workflow Permissions

Set minimal permissions for workflows:

1. Go to: https://github.com/lioriz/short-id/settings/actions
2. **Workflow permissions:**
   - Select **"Read repository contents and packages permissions"**
   - ✅ **"Allow GitHub Actions to create and approve pull requests"** (Optional)
3. **Save**

The release workflow has explicit permissions already configured.

## 5. Repository Settings

### General Security:

1. Go to: https://github.com/lioriz/short-id/settings
2. Under **General:**
   - **Allow merge commits:** Your choice
   - **Allow squash merging:** ✅ Recommended
   - **Allow rebase merging:** ✅ Recommended
3. Under **Danger Zone:**
   - Keep repository private until ready (optional)

## 6. Verify Your Setup

### Test Tag Protection:

```bash
# As a non-admin user (or in a fork):
git clone https://github.com/lioriz/short-id
cd short-id
git tag v999.999.999
git push origin v999.999.999
# Should fail with: "refusing to allow a user to push a protected tag"
```

### Test Branch Protection:

```bash
# Try to push directly to main (should fail):
git checkout main
echo "test" >> README.md
git commit -am "test"
git push origin main
# Should fail with: "refusing to allow a user to push to a protected branch"
```

### Test Release Workflow:

```bash
# As repository admin, test with a pre-release:
git tag v0.0.1-test
git push origin v0.0.1-test

# Watch the workflow:
# https://github.com/lioriz/short-id/actions

# If using environment protection, you'll see:
# "Waiting for approval" - click "Review deployments" to approve
```

## Summary of Protection Layers

| Layer | What It Protects | Who Can Bypass |
|-------|------------------|----------------|
| **Tag Protection** | Creating version tags | Repository admins only |
| **Branch Protection** | Changing Cargo.toml version | PR approval required |
| **Environment Protection** | Publishing to crates.io | Designated approvers only |
| **Repository Check** | Forks triggering releases | Nobody (code check) |
| **Secrets** | crates.io token | Repository admins only |

## Best Practices

1. **Minimum two people:** Always have at least 2 admins who can release
2. **Review releases:** Use environment protection to add a manual review step
3. **Audit logs:** Regularly check Settings → Actions → Logs
4. **Rotate tokens:** Update `CARGO_REGISTRY_TOKEN` every 6-12 months
5. **Monitor releases:** Watch https://github.com/lioriz/short-id/releases
6. **Test in fork:** Test workflow changes in a fork first

## Troubleshooting

### "Tag protection rules don't work"

- Tag protection is a GitHub feature that may not be available on all plans
- Use branch protection + environment protection as alternatives
- The `if: github.repository == 'lioriz/short-id'` check prevents fork abuse

### "I'm an admin but can't push tags"

- Check if you enabled "Restrict who can push to matching branches" and excluded yourself
- Verify you're authenticated with the correct account
- Check Settings → Collaborators to confirm admin status

### "Workflow runs but fails immediately"

- Check the `if: github.repository` condition matches your repo name
- Verify the tag format matches `v*.*.*` pattern
- Check Actions logs for detailed error messages

## Additional Resources

- [GitHub Branch Protection](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches)
- [GitHub Tag Protection](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/managing-repository-settings/configuring-tag-protection-rules)
- [GitHub Environments](https://docs.github.com/en/actions/deployment/targeting-different-environments/using-environments-for-deployment)
- [crates.io Token Management](https://doc.rust-lang.org/cargo/reference/publishing.html#managing-a-cratesio-token)


# Git Commands Quick Reference

## 🚀 Initial Setup (One Time)

```bash
# 1. Navigate to project
cd C:\Users\algmi\source\Rust\file_finder

# 2. Initialize Git (if not done)
git init

# 3. Configure Git (if first time)
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"

# 4. Add remote
git remote add origin https://github.com/yourusername/file_finder.git

# 5. Verify
git remote -v
```

---

## 📤 First Push

```bash
# 1. Stage all files
git add .

# 2. Check what will be committed
git status

# 3. Commit
git commit -m "Initial commit: FileFinder v1.0.0"

# 4. Push to GitHub
git branch -M main
git push -u origin main
```

---

## 🔄 Regular Workflow

```bash
# 1. Make changes
# ... edit files ...

# 2. Check changes
git status
git diff

# 3. Stage changes
git add .
# or stage specific files:
git add src/main.rs

# 4. Commit
git commit -m "Fix: correct banner alignment"

# 5. Push
git push
```

---

## 🏷️ Creating Releases

```bash
# 1. Build release package
build_release.bat
# Enter version when prompted: 1.0.0

# 2. Test the release
# Extract and test release\file_finder-v1.0.0-windows-x64.zip

# 3. Commit any last changes
git add .
git commit -m "Release v1.0.0"
git push

# 4. Create and push tag
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0

# 5. Create Release on GitHub
# - Go to https://github.com/yourusername/file_finder/releases
# - Click "Create a new release"
# - Select tag v1.0.0
# - Upload .zip and .sha256 files
# - Add release notes
# - Publish
```

---

## 🌿 Branching

```bash
# Create new branch
git checkout -b feature/new-feature

# Switch branches
git checkout main
git checkout feature/new-feature

# List branches
git branch

# Delete branch
git branch -d feature/new-feature

# Push branch to GitHub
git push -u origin feature/new-feature
```

---

## ↩️ Undoing Changes

```bash
# Discard changes in working directory
git checkout -- filename

# Unstage file
git reset HEAD filename

# Undo last commit (keep changes)
git reset --soft HEAD~1

# Undo last commit (discard changes)
git reset --hard HEAD~1

# Revert a commit (creates new commit)
git revert <commit-hash>
```

---

## 🔍 Viewing History

```bash
# View commit history
git log

# Compact view
git log --oneline

# View changes in commit
git show <commit-hash>

# View file history
git log -- filename
```

---

## 📥 Syncing with Remote

```bash
# Fetch changes from remote
git fetch origin

# Pull changes and merge
git pull origin main

# Pull and rebase
git pull --rebase origin main
```

---

## 🏷️ Tags

```bash
# List tags
git tag

# Create annotated tag
git tag -a v1.0.0 -m "Release version 1.0.0"

# Create lightweight tag
git tag v1.0.0

# Push tag
git push origin v1.0.0

# Push all tags
git push origin --tags

# Delete tag locally
git tag -d v1.0.0

# Delete tag on remote
git push origin --delete v1.0.0
```

---

## 🔧 Common Issues

### Authentication Failed

**Problem**: Git asks for password, but GitHub no longer accepts passwords.

**Solution**: Use Personal Access Token
1. Go to https://github.com/settings/tokens
2. Click "Generate new token" → "Generate new token (classic)"
3. Select scopes: `repo` (full control)
4. Copy the token
5. Use token as password when Git asks

**Better**: Use SSH
```bash
# Generate SSH key
ssh-keygen -t ed25519 -C "your.email@example.com"

# Add to GitHub
# Copy content of ~/.ssh/id_ed25519.pub
# Paste at https://github.com/settings/keys

# Change remote to SSH
git remote set-url origin git@github.com:yourusername/file_finder.git
```

### Large Files

**Problem**: File is too large for GitHub (>100MB)

**Solution**: Use Git LFS or exclude from repo
```bash
# Option 1: Add to .gitignore
echo "large_file.dat" >> .gitignore

# Option 2: Use Git LFS
git lfs install
git lfs track "*.exe"
git add .gitattributes
```

### Wrong Commit Message

```bash
# Change last commit message
git commit --amend -m "New message"

# Push (if already pushed)
git push --force
```

---

## 📋 File to Include in Git

✅ **Include:**
```
src/
tests/
Cargo.toml
Cargo.lock         (for binaries)
.gitignore
README.md
LICENSE
CHANGELOG.md
CONTRIBUTING.md
EXAMPLES.md
build.bat
build.sh
```

❌ **Exclude (in .gitignore):**
```
target/
*.exe
*.dll
*.so
*.dylib
.vscode/
.idea/
*.swp
*.log
*.json (user data)
```

---

## 🎯 Complete First-Time Setup

```bash
# 1. Clean up and prepare files
cd C:\Users\algmi\source\Rust\file_finder

# 2. Make sure .gitignore is correct
notepad .gitignore

# 3. Replace README with GitHub version
move README.md README_LOCAL.md
move README_GITHUB.md README.md

# 4. Initialize Git
git init
git config user.name "Your Name"
git config user.email "your@email.com"

# 5. Add remote
git remote add origin https://github.com/yourusername/file_finder.git

# 6. Stage and commit
git add .
git commit -m "Initial commit: FileFinder v1.0.0"

# 7. Push
git branch -M main
git push -u origin main

# 8. Create release
build_release.bat
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0

# 9. Upload release on GitHub
# Go to https://github.com/yourusername/file_finder/releases
```

---

## 💡 Tips

1. **Commit often** - Small, focused commits are better
2. **Write good messages** - Future you will thank you
3. **Test before pushing** - Run `cargo test`
4. **Use branches** - For experiments and features
5. **Pull before push** - Avoid conflicts

---

## 🆘 Emergency Commands

```bash
# Abort merge
git merge --abort

# Abort rebase
git rebase --abort

# Save work temporarily
git stash
git stash pop

# Show diff of staged changes
git diff --staged

# Discard all local changes
git reset --hard HEAD
```

---

## 📚 Resources

- [Git Documentation](https://git-scm.com/doc)
- [GitHub Guides](https://guides.github.com/)
- [Git Cheat Sheet](https://education.github.com/git-cheat-sheet-education.pdf)

# Manual Environment Check Commands

Run these commands one by one in Command Prompt:

## Check Rust
```cmd
rustc --version
cargo --version
```

Expected: Version numbers (e.g., rustc 1.80.0)
If not found: Install from https://rustup.rs/


## Check Build Tools
```cmd
where link.exe
```

Expected: Path like C:\Program Files\Microsoft Visual Studio\2022\...
If not found: **THIS IS YOUR PROBLEM!**


## Check Windows SDK
```cmd
dir "C:\Program Files (x86)\Windows Kits\10\Lib\"
```

Expected: List of SDK versions (10.0.xxxxx.0)


## Check environment
```cmd
echo %LIB%
```

Expected: Long path with many directories
If empty: May need to use Developer Command Prompt


## If Build Tools are missing:

1. Download: https://aka.ms/vs/17/release/vs_buildtools.exe

2. Run installer

3. Select: "Desktop development with C++"

4. Click Install (takes 20-40 minutes, 2-6 GB)

5. Reboot computer

6. Run checks again


## After installing Build Tools:

```cmd
cargo clean
cargo build --release
```

Should work without errors!

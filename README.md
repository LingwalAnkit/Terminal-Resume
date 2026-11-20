# Resume Terminal

A terminal-based resume, built with [ratatui](https://ratatui.rs) and crossterm. Run it in a terminal to browse the resume in sections, or print it as plain text.

- Welcome screen, main menu, and six sections: About, Skills, Projects, Experience, Education, Contact
- Keyboard navigation with arrows or j/k, Enter, Esc, and q
- Section flags such as `--projects` to jump straight to a section
- Plain-text output for piping, scripts, and terminals that can't run the interactive view
- Works in Windows Terminal, PowerShell, CMD, macOS Terminal, and Linux terminals

## Requirements

- **Rust** (stable), installed with [rustup](https://rustup.rs). Rust is only needed to build the program. People who receive a prebuilt binary don't need it.
- **Windows:** the MSVC build tools. rustup prompts you to install them, or get them from "Visual Studio Build Tools" with the "Desktop development with C++" workload.

## Run

From the project folder:

```powershell
cargo run
```

Options:

| Command | What it does |
|---|---|
| `cargo run -- --projects` | Opens the Projects section directly. Any of `--about`, `--skills`, `--experience`, `--education`, `--contact` works too. |
| `cargo run -- --plain` | Prints the whole resume as plain text. Add a section flag to print only that section. |
| `cargo run -- --help` | Lists all options. |

Output piped to a file or another program is plain text automatically.

### Keys

| Key | Action |
|---|---|
| Any key | Leave the welcome screen |
| Up / Down, or k / j | Move in the menu, or scroll a section |
| Enter | Open the highlighted section |
| Esc | Go back to the menu from a section |
| q, or Ctrl+C | Quit from any screen |

The window must be at least 60×20. Smaller windows show a resize message.

## Update the resume

All content lives in [src/data.rs](src/data.rs). Edit the strings there, then run `cargo run` to check the result. The program doesn't read the PDF. Run `cargo test` after editing. One test fails if a section is left empty.

Some fields are drafts written from the PDF and may need your own wording: the headline (`role`) and the `summary`. See the notes at the top of `data.rs`.

Links are shown as plain text. The program doesn't open them, because terminals can't reliably do that.

## Test and lint

```powershell
cargo test
```

```powershell
cargo clippy --all-targets -- -D warnings
```

```powershell
cargo fmt
```

## Build a release binary

```powershell
cargo build --release
```

The binary is written to `target\release\resume-terminal.exe` on Windows, or `target/release/resume-terminal` on Linux and macOS. The release profile enables link-time optimisation and strips symbols, so the file is small.

## Distribute a prebuilt binary

Recipients don't need Rust.

### Automatic releases (GitHub Actions)

[.github/workflows/release.yml](.github/workflows/release.yml) builds the program on Windows, Linux, and macOS. To publish a release, push a version tag:

```powershell
git tag v0.1.0
```

```powershell
git push origin v0.1.0
```

The workflow then creates a GitHub Release with `resume-terminal-windows.zip`, `resume-terminal-linux.tar.gz`, and `resume-terminal-macos.tar.gz` attached. Running the workflow by hand from the Actions tab builds the archives without creating a release.

GitHub only shows releases to people who can open the repository. For recruiters to download without an account, the repository has to be public.

### Manual package (Windows)

1. Build the binary: `cargo build --release`.
2. Package it:

   ```powershell
   Compress-Archive -Path target\release\resume-terminal.exe -DestinationPath resume-terminal-windows.zip
   ```

3. Share the zip, for example as an attachment or a GitHub Release asset.

Tell recipients to unzip it, open a terminal in that folder, and run:

```powershell
.\resume-terminal.exe
```

Don't double-click the file. It's a terminal program, so it needs a terminal window.

Unsigned Windows binaries can trigger a SmartScreen warning. The recipient can choose "More info" and then "Run anyway". Code signing removes the warning, but it needs a certificate.

## Project layout

```
resume-terminal/
├── Cargo.toml          dependencies and release settings
└── src/
    ├── main.rs         starts the program, chooses interactive or plain mode
    ├── cli.rs          command-line flags
    ├── app.rs          screen state, navigation, scrolling
    ├── event.rs        maps key presses to actions
    ├── terminal.rs     terminal setup, restore on exit or panic
    ├── section.rs      the six sections, in menu order
    ├── data.rs         resume content
    ├── fallback.rs     plain-text output
    └── ui/
        ├── mod.rs          draws the current screen, small-window check
        ├── welcome.rs      splash screen
        ├── menu.rs         main menu
        ├── section_view.rs section screens, wrapping and scrolling
        ├── widgets.rs      shared frame and footer
        ├── wrap.rs         word wrapping
        └── theme.rs        colours and text styles
```

## Platform notes

The program is developed and tested on Windows. It uses crossterm, which supports Linux and macOS, but those builds haven't been run yet. Please report any problems with terminal display or key handling on those platforms.

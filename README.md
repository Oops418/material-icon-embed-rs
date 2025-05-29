<!-- filepath: /Users/kxyang/Personal/CodeSpaces/material-icon-embed-rs/README.md -->
<div align="center">
    <h1>Material Icon Embed</h1>
    <p>
        <a href="https://github.com/material-extensions/vscode-material-icon-theme"><img src="https://img.shields.io/badge/Material-Rust-blue?logo=Safari" alt="Website"/></a>
    </p>
</div>

## Overview

A Rust library for embedding Material Design icons from the VS Code Material Icon Theme into your applications.

It will automatically update from the upstream [VS Code Material Icon Theme](https://github.com/material-extensions/vscode-material-icon-theme) repository.

## Usage

You can find the mapping of icon names to their SVG files in the [fileIcons.ts](https://github.com/material-extensions/vscode-material-icon-theme/blob/2b43884b90e1873bfd1e68e105f98a787d49f682/src/core/icons/fileIcons.ts) and [folderIcons.ts](https://github.com/material-extensions/vscode-material-icon-theme/blob/2b43884b90e1873bfd1e68e105f98a787d49f682/src/core/icons/folderIcons.ts), or check the source code of the repo. To use this library, add it to your `Cargo.toml`:

```toml
[dependencies]
material-icon-embed-rs = "0.1.0"
```

Then, you can use it in your Rust code like this:

```rust
use material_icon_embed_rs::MaterialIconEmbed;

let icon_path = MaterialIconFile::from_extension("md").unwrap().path();
```

Check the example in the `examples` directory for more details.

## Third-Party Assets

This library uses the following third-party assets:

- [Material Icon Theme (MIT License)](https://github.com/material-extensions/vscode-material-icon-theme)

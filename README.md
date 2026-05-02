# Kore Recipes

![Rust](https://img.shields.io/badge/language-Rust-orange?logo=rust&logoColor=white)
![SQLite](https://img.shields.io/badge/database-SQLite-blue?logo=sqlite&logoColor=white)
![TOML](https://img.shields.io/badge/config-TOML-lightgrey)

This repository manages recipes for **kpm**. Recipes are divided into two main categories: `community` and `official`.

---

## Official Repositories (`official/`)

Unlike the `community` folder, the **official** repository is reserved for applications that meet strict stability and relevance criteria.

### What qualifies as "Official"?

For a recipe to be accepted into `official/`, it must meet the following:

* **Core Tools:** Essential applications for terminal or system workflows (e.g. `helix`, `nvim`, `micro`).
* **Proven Stability:** Only apps with stable releases and active maintenance by their original authors.
* **High Demand:** Widely used software within the Trinity Projects community and Linux users in general.
* **Native Format:** Preference for apps that provide direct binaries or well-structured `tarballs`.

### How to add an official recipe

If you think a package should be official, the structure is identical to others but placed in `recipes/official/`:

```toml
name = "Helix"
package_name = "helix"
url_template = "https://github.com/helix-editor/helix"
description = "A post-modern modal text editor written in Rust with built-in LSP support."
terminal = true
formats = ["tarball"]

[metadata]
maintainer = "The Helix Team"
license = "MPL-2.0 license"
```

---

## Community Repositories (`community/`)

This is where everything else lives. It’s the ideal place for:

* **Games & Launchers:** Like `heroic-games-launcher` or `prism-launcher`.
* **Niche Software:** Tools that not every user needs.
* **New Contributions:** All new packages should start in `community` before being considered for `official`.

---

## Repository Structure

```text
recipes/
├── community/       # Third-party apps, games, and various tools
│   ├── development/
│   └── games/
└── official/        # Essential, stable, and maintained software
    ├── development/ # Editors (Helix, Neovim), languages
    ├── music/       # Basic media players
    ├── network/     # Essential networking tools
    └── utility/     # System utilities
```

---

> **Maintainers Note:** The **Kore Package Manager** team reserves the right to move packages from `community` to `official` based on usage and observed stability.

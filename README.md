# test
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

## Multimedia Support (Icons & Screenshots)

Kore-Recipes and KPM support native multimedia capabilities for the GUI. You can enhance your recipes by adding icons and screenshots.

* **`icon_url`**: A direct link to a PNG or SVG image. KPM will download and display this icon. If omitted, KPM attempts to find a system icon or uses a generic one.
* **`screenshots`**: An array of image URLs (PNG, JPG, WEBP). These will be displayed in an interactive carousel in the KPM GUI.

### Example with Multimedia

```toml
name = "OBS Studio"
package_name = "obs-studio"
url_template = "https://github.com/obsproject/obs-studio/releases/download/30.0.0/obs-studio-x64.tar.gz"
description = "Free and open source software for video recording and live streaming."
category = "Multimedia"

# Multimedia
icon_url = "https://raw.githubusercontent.com/obsproject/obs-studio/master/UI/forms/images/obs.png"
screenshots = [
    "https://obsproject.com/assets/images/new/obs-studio-28-mac.png",
    "https://obsproject.com/assets/images/new/obs-studio-28-windows.png"
]
```

> **Note:** If you want to test your changes locally before pushing, run `cargo run` in the `Kore-Recipes` directory to regenerate your local `packages.db`. You do not need to commit this database; our GitHub Actions workflow will automatically build and publish it upon merging.

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

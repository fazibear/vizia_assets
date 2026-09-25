# vizia_assets

`vizia_assets` generates a `load_assets` function for a Vizia application. It
loads images and fonts from memory and adds stylesheets to the Vizia context.

## Setup

Add the crate as a dependency and declare the asset directory and files in the
application's `Cargo.toml`:

```toml
[dependencies]
vizia_assets = { path = "../vizia_assets" }

[package.metadata.vizia-assets]
asset-dir = "assets"
images = ["*.png"]
fonts = ["*.ttf"]
stylesheets = ["*.css"]
```

The asset directory is relative to the application directory. Asset patterns
are relative to `asset-dir`. Patterns use glob syntax, so `*.png` matches files
directly in `assets/`, while `**/*.png` can match PNG files in nested folders.
Matches are sorted before generating the loader. A wildcard that matches no
files is an error.

Each asset list is optional; omit a list when the application has no assets of
that type. You can also list explicit filenames instead of patterns:

```toml
[package.metadata.vizia-assets]
asset-dir = "assets"
images = ["background.png", "knob.png"]
fonts = ["exquisite_corpse.ttf"]
stylesheets = ["base.css", "meters.css"]
```

## Loading assets

Call `load_assets` once when setting up the Vizia context:

```rust
use vizia_assets::load_assets;

fn setup(cx: &mut vizia_plug::vizia::prelude::Context) {
    load_assets(cx).expect("assets loaded");
}
```

Image keys are the paths relative to `asset-dir`, with a trailing `.png`
removed. For example, `icons/power.png` is registered as `icons/power`.
Fonts are added from embedded bytes, and stylesheets are embedded in the binary.

The build script finds the consuming manifest near Cargo's build output
directory. Keeping the configuration under `[package.metadata.vizia-assets]`
avoids Cargo's unused manifest key warning and works with path and Git
dependencies.

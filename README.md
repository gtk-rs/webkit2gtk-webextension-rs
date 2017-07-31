# webkit2gtk-webextension [![Build Status](https://travis-ci.org/gtk-rs/webkit2gtk-webextension-rs.png?branch=master)](https://travis-ci.org/gtk-rs/webkit2gtk-webextension-rs) [![Gitter](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/gtk-rs/gtk)

[Project site](http://gtk-rs.org/) | [Online documentation](http://gtk-rs.org/docs/)

__Rust__ bindings and wrappers for __webkit2gtk-webextension__.

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](http://gtk-rs.org/#using).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
webkit2gtk-webextension-rs = { git = "https://github.com/gtk-rs/webkit2gtk-webextension-rs.git" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
gtk = "0.2"
webkit2gtk-webextension-rs = { git = "https://github.com/gtk-rs/webkit2gtk-webextension-rs.git" }
```

## Contribute

Contributor you're welcome!

## License

__webkit2gtk-webextension-rs__ is available under the MIT License, please refer to it.

# config-dirs

Load a config file by trying out default config file locations:

- `{NAME_UPPERCASE}_CONFIG` envitonment variable
- `~/.config/{name}/config.toml`
- `/etc/{name}/config.toml`
- `/usr/local/etc/{name}/config.toml`
- `~/Library/Preferences/{name}/config.toml`
- `/usr/local/etc/{name}/config.toml`

```rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Config {}

let config: Config = config_dirs::load("my-app", toml::from_str).expect("Failed to load config");
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in config-dirs by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

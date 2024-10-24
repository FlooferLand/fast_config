## A tutorial on how to convert your data to the newest version!
This file targets helping you switch to the newest version, from the last version.

___Note:___ You're able to look at the [commit history of this file](https://github.com/FlooferLand/fast_config/commits/main/CONVERSION_TUTORIAL.md) to see past versions of this file, for older versions of the crate.

This tutorial currently targets conversion from **1.1.x** to **1.2.0**.

# Changes
___Note:___ Always check the [GitHub version](https://github.com/FlooferLand/fast_config/blob/main/CONVERSION_TUTORIAL.md) of this file,
as it's the most up to date.
---

1.2.0 introduces ordinary JSON using [serde_json](https://crates.io/crates/serde_json) and moves away from JSON5, as the [json5](https://crates.io/crates/json5) crate this project uses hasn't received an update in ~3 years. <br/>
I don't plan on deprecating JSON5 as that crate still works well enough, though I might move to a better crate in the future.

### Renames / changes

---
- `ConfigSetupOptions.save_on_drop` is now deprecated since it can lead to unsafe I/O while your program is exiting. Just use `Config::save` instead. _(thanks to [@bobhy](https://github.com/bobhy) for bringing this ancient piece of code to my attention)_
---

## That's it!

You can view the [examples](./examples) directory for examples regarding the new syntax.

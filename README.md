# Next semver

This is an extremely simple service. You send it your current version and the type of bump you want, and you get back the new version.

* `/minor/1.2.4` -> `1.3.0`
* `/patch/0.3.7-alpha` -> `0.3.8-alpha`

I have this as a http endpoint to keep my CI pipelines code-free.

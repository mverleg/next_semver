# Next semver

This is an extremely simple service. You send it your current version and the type of bump you want, and you get back the new version.

* `/minor/1.2.4` -> `1.3.0`
* `/patch/0.3.7-alpha` -> `0.3.8-alpha`

I have this as a http endpoint to keep my CI pipelines code-free.

## Run locally

You can run with just Docker (the image is 2MB):

    docker run -p8080:8080 -it mverleg/next_semver:latest

Or you can build it yourself with Cargo:

    ROCKET_ADDRESS="0.0.0.0" ROCKET_PORT=8080 cargo run --features=web --bin next_semver    


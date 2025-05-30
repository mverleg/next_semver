# Next semver

This is an extremely simple service. You send it your current version and the type of bump you want, and you get back the new version.

* `/minor/1.2.4` -> `1.3.0`
* `/patch/0.3.7-alpha` -> `0.3.8-alpha`

I have this as a http endpoint to keep my CI pipelines code-free.

## Webservice

No guarantees about uptime, but I intend to have the service occasionally available at [next.tryin.top](https://next.tryin.top). Example:

    curl -f https://next.tryin.top/minor/v1.2.4
    # 1.3.0

## Run locally

You can run with just Docker ([the image](https://hub.docker.com/repository/docker/mverleg/next_semver) is 2MB):

    docker run -p8080:8080 -it mverleg/next_semver:latest

Or you can build it yourself with Cargo, after checking out the code:

    ROCKET_ADDRESS="0.0.0.0" ROCKET_PORT=8080 cargo run --features=web --bin next_semver    

## Crate

The crate is available as [next_semver](https://crates.io/crates/next_semver) and can be used as a library (without web dependencies).

## Deploy

To preview the generated yaml:

    helm template --debug -n next-semver next-semver helm

To deploy or upgrade:

    # dev
    helmf next-semver-dev helm prod=false domain=next-dev.tryin.top

    # prod
    helmf next-semver helm prod=true domain=next.tryin.top



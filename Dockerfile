
FROM mverleg/rust_nightly_musl_base:nodeps_2022-01-13 AS build

ARG BIN=next_semver

# Copy the code (all except .dockerignore).
COPY ./ ./

# Check
RUN cargo test --all-features
RUN cargo --offline clippy --all-features --tests -- -D warnings
RUN cargo --offline fmt --all -- --check

# Build (for release)
RUN find . -name target -prune -o -type f &&\
    touch -c build.rs src/main.rs src/lib.rs &&\
    cargo --offline build --bin "$BIN" --all-features --release --locked

# Copy executable
RUN find . -wholename "*/target/*" -name "$BIN" -type f -executable -print -exec cp {} /"$BIN" \; &&\
    test -f /"$BIN"

# Second stage image to decrease size
FROM scratch AS executable

ARG BIN=next_semver

ENV PATH=/
ENV RUST_BACKTRACE=1
ENV RUST_LOG='warn'
ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT=8080
WORKDIR /code

COPY --from=build /"$BIN" /"$BIN"

ENTRYPOINT ["next_semver"]


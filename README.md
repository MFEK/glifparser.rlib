# glifparser docs

Docs start @ <./glifparser/>.

## Build process

1. Docs are Cargo-generated as:
    ```bash
    cargo doc --no-deps --features=fat
    ```
2. Then flip to `gh-pages`.
3. Copy to some temporary place…and delete everything.
    ```bash
    cp -r target/doc /tmp/GLIFdoc
    rm -rf ./*
    ```
4. Get everything back (this prevents deleted files from lingering forever.)
    ```bash
    rsync -Paz /tmp/GLIFdoc/ ../$(basename $PWD)/
    ```

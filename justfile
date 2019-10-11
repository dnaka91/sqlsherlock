os := os()

build:
    #!/usr/bin/env sh
    if [ {{os}} = "macos" ]; then
        export PKG_CONFIG_PATH=/usr/local/opt/sqlite/lib/pkgconfig:/usr/local/opt/mysql/lib/pkgconfig:/usr/local/opt/postgresql/lib/pkgconfig
        cargo clean
        cargo build
    fi

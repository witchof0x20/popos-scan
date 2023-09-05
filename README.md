# popos-scan

Scans the official PopOS repositories (https://apt.pop-os.org/) to determine what kernel/driver versions are available, and this, what is "stable".

This is very early, not optimized. All it does is grab manifests, but please don't run it over and over and ddos their servers.

## Running
```sh
cargo run
```
or
```
nix run
```

probably requires openssl as a dependency. might switch to webpki and avoid this

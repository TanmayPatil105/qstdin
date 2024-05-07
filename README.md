## qstdin

a simple interface for querying stdin

### Install

```console
$ cargo add qstdin
```

### Testing

```console
$ cargo run --example qstdin
input? true
file? false
directory? false
```

```console
$ cargo run --example qstdin < Cargo.toml
input? false
file? true
directory? false
```

```console
$ cargo run --example qstdin < src
input? false
file? false
directory? true
```

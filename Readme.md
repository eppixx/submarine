# Submarine

Submarine is a library for sending requests to a [subsonic](http://www.subsonic.org/pages/index.jsp) server for rust code.
It implements everything for version v1.16.1.

There are other servers that implement more or less the same interface.
Submarine strives to incorporate these differences.

## Implemented differences so far

As implementations differ from the subsonic interface, others can specifically activated by turning on a feature like in the following example `Cargo.toml`:

```toml
# Cargo.toml
...
[dependencies]
submarine = { version = "0.1.0", features = ["navidrome"] }
...
```

### Navidrome

The navidrome api can be seen [here](https://www.navidrome.org/docs/developers/subsonic-api/).
The feature flag is called `navidrome`.

# cargo2pkg

This is a simple command line to generate a FreeBSD pkg +MANIFEST from a cargo project to std.out.

The intention is to use it with `pkg create` to simplify the distribution of Rust application on FreeBSD.
See the [pkg create docs](https://www.freebsd.org/cgi/man.cgi?pkg-create(8)) for more background.

**N.B. UNDER DEVELOPMENT**
While it does not change or create any files in your environment this tool has not been exercised further than the unit tests.

Properties Supported:

- [x] name
- [ ] origin
- [x] version
- [ ] comment
- [x] maintainer
- [ ] abi
- [ ] arch
- [ ] www
- [ ] prefix
- [x] flatsize
- [ ] deps
- [ ] desc
- [x] files
- [ ] conflict
- [ ] option
- [ ] dir

### usage

Sample usage scenario 
```
% git clone https://github.com/no9/cargo2pkg.git
% cd cargo2pkg
% cargo build --release
% ./target/release/cargo2pkg

{
  "name": "cargo2pkg",
  "origin": "",
  "version": "0.1.0",
  "comment": "",
  "maintainer": "Anton Whalley <anton@venshare.com>",
  "abi": "",
  "arch": "",
  "prefix": "",
  "flatsize": 3443712,
  "desc": "",
  "files": {
    "cargo2pkg": "177d55ea1d9883ec48134b328c4cf0296ace6aef99fe79ed13b71436de5e7a74"
  }
}

```
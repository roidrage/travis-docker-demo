# Building a multi-platform release with Travis & Docker

This repo demonstrates how to test a Rust binary on both 32-bit and 64-bit platforms, and if the tests pass, release it to GitHub Releases.

All of the important details are in [.travis-yml](https://github.com/joecorcoran/travis-docker-demo/blob/master/.travis.yml).

It relies on [cross](https://github.com/japaric/cross) for Docker-based cross-platform compilation.

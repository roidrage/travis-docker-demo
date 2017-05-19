# Building a multi-platform release with Travis & Docker

This repo demonstrates how to test a Rust binary on both 32-bit and 64-bit platforms, and if the tests pass, release it to GitHub Releases.

All of the important details are in [.travis-yml](https://github.com/joecorcoran/travis-docker-demo/blob/master/.travis.yml).

It relies on [cross](https://github.com/japaric/cross) for Docker-based cross-platform compilation.

## Before build stages

It has been possible to deploy from mutiple jobs to the same GitHub release for some time. [This video](https://vimeo.com/218167319) shows a build being broken on one of two targeted Linux architectures. Later the build is fixed, and the eventual release contains binaries for both targets.

Although it's nifty, the problem with this approach is that [the jobs](https://github.com/joecorcoran/travis-docker-demo/blob/e54e2d665f18f07c59b00b2826a0037cfec52dcc/.travis.yml) have no awareness of each other's status – if one job fails, a passing job will still [go ahead and deploy](https://github.com/joecorcoran/travis-docker-demo/releases/tag/build-18) its part of the multi-platform release.

## After build stages

By [changing .travis.yml](https://github.com/joecorcoran/travis-docker-demo/blob/baeba2572038a7b2c2c1d8a364b136eef453dc4d/.travis.yml) to use [build stages](https://docs.travis-ci.com/user/build-stages/) we can specify a stage with two jobs to run the tests, followed by a deployment stage that also has two jobs, but which [depends on the success of the previous stage](https://travis-ci.org/joecorcoran/travis-docker-demo/builds/234037222). [This video](https://vimeo.com/218169173) shows the jobs running in separate stages.

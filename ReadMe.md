sio-smoltcp.rs [![Unlicensed work](https://raw.githubusercontent.com/unlicense/unlicense.org/master/static/favicon.png)](https://unlicense.org/)
==============
~~![GitLab Build Status](https://gitlab.com/KOLANICH-libs/sio-smoltcp.rs/badges/master/pipeline.svg)~~
~~![GitLab Coverage](https://gitlab.com/KOLANICH-libs/sio-smoltcp.rs/badges/master/coverage.svg)~~
~~[![GitHub Actions](https://github.com/KOLANICH-libs/sio-smoltcp.rs/workflows/CI/badge.svg)](https://github.com/KOLANICH-libs/sio-smoltcp.rs/actions/)~~
[![Libraries.io Status](https://img.shields.io/librariesio/github/KOLANICH-libs/sio-smoltcp.rs.svg)](https://libraries.io/github/KOLANICH-libs/sio-smoltcp.rs)
[![Code style: antiflash](https://img.shields.io/badge/code%20style-antiflash-FFF.svg)](https://codeberg.org/KOLANICH-tools/antiflash.py)

**We have moved to https://codeberg.org/KOLANICH-libs/sio-smoltcp.rs, grab new versions there.**

Under the disguise of "better security" Micro$oft-owned GitHub has [discriminated users of 1FA passwords](https://github.blog/2023-03-09-raising-the-bar-for-software-security-github-2fa-begins-march-13/) while having commercial interest in success and wide adoption of [FIDO 1FA specifications](https://fidoalliance.org/specifications/download/) and [Windows Hello implementation](https://support.microsoft.com/en-us/windows/passkeys-in-windows-301c8944-5ea2-452b-9886-97e4d2ef4422) which [it promotes as a replacement for passwords](https://github.blog/2023-07-12-introducing-passwordless-authentication-on-github-com/). It will result in dire consequencies and is competely inacceptable, [read why](https://codeberg.org/KOLANICH/Fuck-GuanTEEnomo).

If you don't want to participate in harming yourself, it is recommended to follow the lead and migrate somewhere away of GitHub and Micro$oft. Here is [the list of alternatives and rationales to do it](https://github.com/orgs/community/discussions/49869). If they delete the discussion, there are certain well-known places where you can get a copy of it. [Read why you should also leave GitHub](https://codeberg.org/KOLANICH/Fuck-GuanTEEnomo).

---

[Sans-IO-style](https://sans-io.readthedocs.io/how-to-sans-io.html) [FFI](https://en.wikipedia.org/wiki/Foreign_function_interface) for [smoltcp](https://github.com/smoltcp-rs/smoltcp).

Obviously, this lib wouldn't be possible without `smoltcp` existence.

This library reuses some portions of `smoltcp` source code.


Limitations
===========
* We copy bytes back and forth, which is inefficient
* Currently it is early beta, not all the features are implemented, and API is not yet stabilized.

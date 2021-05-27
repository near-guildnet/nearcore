<br />
<br />

<p align="center">
<img src="docs/logo.svg" width="240">
</p>

<br />
<br />


## Reference implementation of NEAR Protocol

[![Build Status][ci-badge-master]][ci-url] 
![Stable Status][stable-release]
![Prerelease Status][prerelease]
[![codecov][codecov-badge]][codecov-url]
[![Discord chat][discord-badge]][discord-url]
[![Telegram Group][telegram-badge]][telegram-url]

[stable-release]: https://img.shields.io/github/v/release/nearprotocol/nearcore?label=stable
[prerelease]: https://img.shields.io/github/v/release/nearprotocol/nearcore?include_prereleases&label=prerelease
[ci-badge-master]: https://badge.buildkite.com/a81147cb62c585cc434459eedd1d25e521453120ead9ee6c64.svg?branch=master
[ci-url]: https://buildkite.com/nearprotocol/nearcore
[codecov-badge]: https://codecov.io/gh/nearprotocol/nearcore/branch/master/graph/badge.svg
[codecov-url]: https://codecov.io/gh/nearprotocol/nearcore
[discord-badge]: https://img.shields.io/discord/490367152054992913.svg
[discord-url]: https://near.chat
[telegram-badge]: https://cdn.jsdelivr.net/gh/Patrolavia/telegram-badge@8fe3382b3fd3a1c533ba270e608035a27e430c2e/chat.svg
[telegram-url]: https://t.me/cryptonear

## About NEAR

NEAR's purpose is to enable community-driven innovation to benefit people around the world.

To achieve this purpose, *NEAR* provides a developer platform where developers and entrepreneurs can create apps that put users back in control of their data and assets, which is the foundation of ["Open Web" movement][open-web-url].

One of the components of *NEAR* is NEAR Protocol, an infrastructure for server-less applications and smart contracts powered by blockchain.
NEAR Protocol is built to deliver usability and scalability of modern PaaS like Firebase at fraction of prices that blockchains like Ethereum charge.

*NEAR* overall provides wide range of tools for developers to easily build applications:
 - [JS Client library][js-api] to connect to NEAR Protocol from your applications.
 - [Rust][rust-sdk] and [AssemblyScript][as-sdk] SDKs to write smart contracts and stateful server-less functions.
 - [Numerous examples][examples-url] with links to hack on them right inside your browser.
 - [Lots of documentation][docs-url], with [Tutorials][tutorials-url] and [API docs][api-docs-url].

[open-web-url]: https://techcrunch.com/2016/04/10/1301496/ 
[js-api]: https://github.com/near-guildnet/near-api-js 
[rust-sdk]: https://github.com/near/near-sdk-rs
[as-sdk]: https://github.com/near/near-sdk-as
[examples-url]: https://near.dev
[docs-url]: http://docs.nearprotocol.com
[tutorials-url]: https://docs.nearprotocol.com/docs/roles/developer/tutorials/introduction
[api-docs-url]: https://docs.nearprotocol.com/docs/roles/developer/examples/nearlib/introduction

## Join The GuildNet Network

The easiest way to join GuildNet, is by using `nearup` command, which you can install:

```bash
curl --proto '=https' --tlsv1.2 -sSfL https://raw.githubusercontent.com/near-guildnet/nearup/master/nearup | python3
```

You the network:
* GuildNet: `nearup guildnet`


Check `nearup` repository for [more details](https://github.com/near-guildnet/nearup) how to run with or without docker.

To learn how to become validator, checkout [documentation](https://docs.nearprotocol.com/docs/validator/staking-overview).


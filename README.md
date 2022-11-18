![Server Build Status](https://img.shields.io/github/workflow/status/lyptt/chameleon/CI?label=server%20build) ![Server Build Status](https://img.shields.io/github/workflow/status/lyptt/chameleon/CI%20Web?label=ui%20build) ![Open issues](https://img.shields.io/github/issues-raw/lyptt/chameleon?color=%2300cc00)

Chameleon is a **free, open source social network server** based on ActivityPub where users can share photos with friends and followers, and discover a feed of content tailored to their interests. All Chameleon servers are interoperable and part of the Fediverse, allowing for multiple servers to share each other's content as part of a federated network.

Chameleon's default language is ActivityPub, allowing for any server that speaks ActivityPub to expose its content to Chameleon, and vice versa.

This project is in the very early planning stages, and will evolve rapidly as time goes on. PRs and contributors are welcome to pitch in and create a truly open and decentralized alternative to popular photo sharing social networks.

# Planned Features

## No vendor lock-in: Fully interoperable with any ActivityPub server

Much like Mastodon and other Fediverse servers, Chameleon will interoperate with any server that speaks ActivityPub

## Real-time chronological timeline

A core goal of Chameleon is to not deliver a curated timeline like popular locked down social networks. You will only ever see a chronological timeline, with the power to curate your timeline as you please.

## Data portability built in

Data portability is essential in the modern age of social networks. You can download original high resolution versions of all of your photos, along with metadata in a simple, well-defined and backwards compatible format.

Data transfer between Chameleon instances will be built-in, allowing you to move all of your data to another server with ease, or permanently delete your data at any time.

## Safety and moderation tools

Chameleon will include tools for system administrators to decide which Fediverse servers can interact with their Chameleon server, and users will be able to have comprehensive control over the privacy of their data, and what kind of posts they'll see in their feed.

## Comprehensive REST API

Chameleon will provide its own API that's easy for developers to consume and build applications and automations around. This will stand alongside the built-in ActivityPub functionality.

# Deployment

## Tech stack:

- Rust powers the API and server-side functionality, and is used for any web-based tasks and background jobs
- React provides the default web UI, but server administrators are free to plug in their own UI and configure it to interoperate with the Chameleon server at any time

## Requirements:

- PostgreSQL 14, older versions may work but aren't tested against
- AWS SQS, other job queue technologies may be supported in the future e.g. RabbitMQ
- Redis

# Contributing

Chameleon is **free, open source software** licensed under **AGPLv3**.

Contributions are welcomed at any time, feel free to raise PRs or feature suggestions.

Copyright (C) 2022 lyptt

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see https://www.gnu.org/licenses/.

# Licensing

A full list of third party licenses can be produced via the following command:

```bash
cargo-about generate -c=about.toml docs/license.hbs > THIRD_PARTY.md
```

LICENSE.md and THIRD_PARTY.md are included with all releases.

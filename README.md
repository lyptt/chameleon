# Orbit

![Server Build Status](https://img.shields.io/github/actions/workflow/status/lyptt/orbit/ci.yml?label=server%20build) ![Server Build Status](https://img.shields.io/github/actions/workflow/status/lyptt/orbit/ci-web.yml?label=ui%20build) ![Open issues](https://img.shields.io/github/issues-raw/lyptt/orbit?color=%2300cc00)

<img src="./public/static/images/logo.svg" width="128" alt="Orbit">

Orbit is a **free, open source social network server** where users can post on public, federated forums, and discover a feed of content tailored to their interests. All Orbit servers are interoperable and part of the Fediverse, allowing for multiple servers to share each other's content as part of a federated network.

Orbit speaks ActivityPub, an open federation protocol powering many Fediverse applications.

This project is in the very early development stages, and will evolve rapidly as time goes on. PRs and contributors are welcome to pitch in and create a truly open and decentralized alternative to popular photo sharing social networks.

# Supported Features

## No vendor lock-in: Fully interoperable with any ActivityPub server

Much like Mastodon and other Fediverse servers, Orbit interoperates with any server that speaks ActivityPub.

## Real-time chronological timeline

A core goal of Orbit is to not deliver a curated timeline like popular locked down social networks. You will only ever see a chronological timeline, with the power to curate your timeline as you please.

# Planned Features

## Data portability built in

Data portability is essential in the modern age of social networks. You will be able to download original high resolution versions of all of the multimedia attached to your posts, along with metadata and content in a simple, well-defined and backwards compatible format.

Data transfer between Orbit instances will be built-in, allowing you to move all of your data to another server with ease, or permanently delete your data at any time.

## Safety and moderation tools

Orbit will include tools for system administrators to decide which Fediverse servers can interact with their Orbit server, and users have comprehensive control over the privacy of their data, and will have control over what kind of posts they'll see in their feed.

## Comprehensive REST API

Orbit provides its own API that's easy for developers to consume and build applications and automations around. This will stand alongside the built-in federation functionality.

# Deployment

## Tech stack:

- Rust powers the API and server-side functionality, and is used for any web-based tasks and background jobs
- React provides the default web UI (via NextJS + TypeScript), but server administrators are free to plug in their own UI and configure it to interoperate with the Orbit server at their leisure
- Orbit does not mandate any one application to interact with a Orbit instance, and no application is 'special', the default web UI uses exactly the same APIs and access controls as third party Orbit-supported apps.

## Requirements:

- **A relational DB**. Currently we only support PostgreSQL 14. Older versions may work but aren't tested against. Other DBs are not planned to be supported in the medium to long term future.
- **A job queue**. Currently we support AWS SQS, and RabbitMQ. For SQS you must create the queues yourself, for RabbitMQ they're created on startup.
- **An in-memory DB**. Currently we only support Redis.
- **A CDN**. Currently we support AWS S3, and any file store accessible from your instance's server via a local file path.

# Contributing

Orbit is **free, open source software** licensed under **AGPLv3**.

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

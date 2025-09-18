<div align="center">
<img width="128" height="128" alt="image" src="https://github.com/user-attachments/assets/1c69fbca-4a6e-4334-8991-6ea1768811ab" />

<h1>Regionoix</h1>

[www.regionoix.gasdev.fr](https://www.regionoix.gasdev.fr)

E-Commerce website made for a 2 weeks school project.

_High quality French regional products reseller, world-wide._

</div>

## Technologies

| Stack Tier                  | Technologies                                                                                                                                                                                                                                                                                     |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Frontend (Typescript)**   | [Angular](https://angular.dev/) / [TailwindCSS](https://tailwindcss.com/) / [DaisyUI](https://daisyui.com/) / [FontAwesome](https://github.com/FortAwesome/angular-fontawesome) / [OpenAPI generator CLI](https://github.com/OpenAPITools/openapi-generator-cli)                                 |
| **Backend (Rust)**          | [actix-web](https://actix.rs/) / [sea-orm](https://www.sea-ql.org/SeaORM/) / [utoipa](https://github.com/juhaku/utoipa) / [swagger-ui](https://github.com/juhaku/utoipa/tree/master/utoipa-swagger-ui) / [async-stripe](https://payments.rs/)                                                    |
| **System Services (NixOS)** | [Garage S3](https://garagehq.deuxfleurs.fr/) / [MeiliSearch](https://www.meilisearch.com/) / [Caddy](https://caddyserver.com) / [Beszel](https://beszel.dev/*) / [Uptime Kuma](https://github.com/louislam/uptime-kuma) / [PostgreSQL](https://www.postgresql.org/) / [Redis](https://redis.io/) |

## Architecture

![Regionoix Architecture Mermaid Chart](https://github.com/user-attachments/assets/b03d7909-d8b8-497d-9b53-9b4056f7a9f2)

## Showcase

<figure>
    <img width="4184" height="3176" alt="Regionoix search page" src="https://github.com/user-attachments/assets/edf3359e-2d72-4691-ad11-42d057a78f1d" />
    <figcaption>Search page</figcaption>
</figure>
<br>

<figure>
    <img width="4184" height="1994" alt="Regionoix product (wine) page" src="https://github.com/user-attachments/assets/a0961ec9-1126-4119-bc10-4ad32cac7cb2" />
    <figcaption>Product page</figcaption>
</figure>
<br>

<figure>
    <img width="4184" height="1994" alt="Regionoix account page" src="https://github.com/user-attachments/assets/186c4a0b-b88a-4662-9f2b-c624ba32a981" />
    <figcaption>Account page</figcaption>
</figure>
<br>

<figure>
    <img width="4184" height="1994" alt="Regionoix basket page" src="https://github.com/user-attachments/assets/eb3e9387-f7e4-4256-bbaa-06f3e55f7811" />
    <figcaption>Basket page</figcaption>
</figure>
<br>

## Benchmarks / Load testing

Load testing was done using the [drill](https://crates.io/crates/drill) tool
using the [backend/benchmark.yml](./backend/benchmark.yml) config.

| Metric                    | Results        |
| ------------------------- | -------------- |
| Time taken for tests      | 124.0 seconds  |
| Total requests            | 110000         |
| Successful requests       | 80000          |
| Failed requests (non 200) | 30000          |
| Requests per second       | 886.95 [#/sec] |
| Median time per request   | 15ms           |
| Average time per request  | 18ms           |
| Sample standard deviation | 9ms            |
| 99.0'th percentile        | 72ms           |
| 99.5'th percentile        | 76ms           |
| 99.9'th percentile        | 83ms           |

> NOTE:
> Failed requests are requests with non 200 status codes.
> They are deliberate and due to non-authenticated/invalid requests,
> they still are handled by the server.

These were ran against a 6c Intel Haswell (no TSX) @ 2.399GHz VPS with 11679MiB
of RAM and SSD storage.

The system CPU usage peaked at 46.78% and total system RAM usage stayed under 1GB.

These results show that our system can handle almost 1K req/sec without
overloading the system while still achieving low latency responses. The Actix
CPU cores config could even be doubled (uses systems core count as default
thread count) to max out CPU usage.

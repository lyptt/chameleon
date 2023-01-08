# Open / Non-cloud Deployment

This folder contains sample configuration files you can use to deploy Orbit to a non-AWS environment.

You can use these files to deploy an Orbit stack making use of:

- Nginx reverse proxy for the API and default UI
- RabbitMQ for background jobs
- PostgreSQL 15 database

It's recommended to adjust these configuration files as needed.

## Nginx Configuration

No assumption has been made on how you handle SSL. HTTPS is required for Orbit to federate to other Orbit instances, however if you don't have any need for federation, it's not required.

Lets Encrypt is recommended to handle HTTPS for you, and certbot can automatically update your Nginx configuration to ensure Nginx serves valid SSL certificates for your Orbit instance.

Orbit serves secure pages for OAuth authentication and the assets for them directly from the API server, and the .well-known path is also handled by the API, so multiple proxy passes are needed.

If you want more fine grained routing, please ensure the following routes proxy pass to the API:

- `/api/**/*`
- `/.well-known/nodeinfo`
- `/.well-known/webfinger`
- `/.well-known/status`
- `/.well-known/host-meta`
- `/.well-known/webfinger`

The remaining routes should proxy pass to the default UI.

## Status checks

If you're using a load balancer, you can make use of the `/.well-known/status` API to flag whether the API is functioning correctly. A `200` response code indicates that the API is functioning in your environment.

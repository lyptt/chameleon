# Testing Federation

Testing federation is fairly difficult and involves a quite involved setup. This document details all of the steps you need
to undertake to be able to test federating between two local orbit instances.

All of the below instructions assume you're using the config files present in the orbit and orbit-2 folders.

## Infrastructure

### Database setup

To start, you'll need a PostgreSQL 15 server running locally.

You'll need two databases:

- orbit
- orbit2

You'll also need a DB user 'orbit' with the password 'orbit' that has full admin access to the above databases with the ability to create / update / delete tables, rows, constraints and indexes.

You should now either manually run each SQL script in the db folder in order against both databases, or you can follow the database migrations guide and migrate each database in turn.

### Queue setup

Next, you'll need a RabbitMQ instance running locally. It's expected that you can connect to RabbitMQ and declare queues, exchanges, and create and get jobs without authentication. No special exchange types need to be configured.

### CDN setup

The configuration for both instances uses S3 with the same bucket and CloudFront distribution to keep things simple, so you'll need an AWS account, an S3 bucket and a CloudFront distribution serving content from that bucket.

Put your CloudFront distrubution base URL in orbit/config/default.toml and orbit-2/config/default.toml, along with your S3 bucket name and an AWS access key and secret key.

### Reverse proxy and local access

You'll need to install dnsmasq to be able to map .test domains to localhost. Configure your local dnsmasq to allow for any .test domain to map to localhost and ensure dig queries and pings all point to localhost.

Next, you'll need Nginx installed and working. You should generate a self-signed SSL cert following a guide like [this one](https://www.howtogeek.com/devops/how-to-create-and-use-self-signed-ssl-on-nginx/).

Place the orbit/nginx.conf file in your Nginx sites-available folder as 'orbit', and orbit-2/nginx.conf file in your Nginx sites-available folder as 'orbit2'. Symlink the two files to your sites-enabled folder to enable them.

## Running both instances

### Web UI

The Web UI is the unfortunate painful part of this, as NextJS will bake all environment variables used for the API URL, CDN, etc directly into the build files in the .next folder inside the ui-web folder, so you'll need to duplicate the ui-web folder to somewhere else in your system.

Within your first ui-web folder, configure your environment and run the ui in dev mode from one terminal window as shown below:

```bash
export NEXT_PUBLIC_API_URI=https://orbit.test/api
export NEXT_PUBLIC_FQDN=https://orbit.test
export OAUTH_REDIRECT_URI=https://orbit.test/api/auth/authorize
export PORT=3000
yarn
yarn dev
```

Within your second ui-web folder, configure your environment and run the ui in dev mode from another terminal window as shown below:

```bash
export NEXT_PUBLIC_API_URI=https://orbit2.test/api
export NEXT_PUBLIC_FQDN=https://orbit2.test
export OAUTH_REDIRECT_URI=https://orbit2.test/api/auth/authorize
export PORT=3001
yarn
yarn dev
```

### API

You'll now need four more terminal windows, two for the worker instances and two for the API instances. Each set of terminal commands is for one of each of your terminal windows.

#### Terminal 1 - API instance 1

```bash
cd docs/config/orbit
../../../target/debug/orbit
```

#### Terminal 2 - Worker instance 1

```bash
cd docs/config/orbit
../../../target/debug/orbit-worker
```

#### Terminal 3 - API instance 2

```bash
cd docs/config/orbit-2
../../../target/debug/orbit
```

#### Terminal 4 - Worker instance 2

```bash
cd docs/config/orbit-2
../../../target/debug/orbit-worker
```

## Testing it works

Go to https://orbit.test and https://orbit2.test in a web browser. Confirm that everything is loading and accept any SSL warnings that pop up. You may need to make sure that the XHR requests to the API aren't also being blocked due to SSL errors.

Register a user on each instance, making sure that you choose a unique handle for each instance to make debugging easier.

You should now be able to create orbits on each instance and federate between instances via the built-in search functionality.

To search for a remote user, you can use @handle@domain.name, and to search for a remote orbit you can use o/shortcode@domain.name.

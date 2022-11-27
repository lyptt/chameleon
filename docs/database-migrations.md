# Database Migrations

Chameleon makes use of contiguous migration scripts for every schema change, meaning that in order to successfully migrate your database to the latest schema version, you should run the scripts in order from V1 to the highest version number.

For an easy way to migrate your database, you can download [refinery](https://github.com/rust-db/refinery/releases/latest), and follow the instructions below to update your database.

## 1. Updating your path

In order to run refinery from a command prompt / terminal, you need to add it to your PATH, along with the DB URL you'll be using.

### Windows

On Windows, you should add the directory containing `refinery.exe` to your PATH by following [these instructions](https://windowsloop.com/how-to-add-to-windows-path/).

You should also add a DB_URL environment variable containing the URL of the Postgres DB you're migrating, e.g. `postgresql://ADMIN_USERNAME:ADMIN_PASSWORD@DATABASE_IP_ADDRESS_OR_URL:5432/DATABASE_NAME`.

### macOS / Linux

On Unix-like OSes, it depends on what shell you're using.

For bash or bash-like shells, you can update your environment by adding the following lines to your ~/.bashrc file:

```bash
export PATH=$PATH:/path/to/refinery
export DB_URL=postgresql://ADMIN_USERNAME:ADMIN_PASSWORD@DATABASE_IP_ADDRESS_OR_URL:5432/DATABASE_NAME
```

For fish shells, you can update your environment by adding the following lines to your ~/.config/fish/config.fish file:

```fish
set -gx PATH $PATH /path/to/refinery
set -gx DB_URL postgresql://ADMIN_USERNAME:ADMIN_PASSWORD@DATABASE_IP_ADDRESS_OR_URL:5432/DATABASE_NAME
```

## 2. Run schema migration scripts

Run the following commands from a command prompt / terminal to migrate your DB to the latest version:

```bash
cd /path/to/chameleon/repository
cd db
refinery migrate -e DB_URL -p db
```

CREATE TABLE tombstones (
  tombstone_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  deleted_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  fediverse_uri varchar(64) NOT NULL,
  former_type varchar(64) NOT NULL,
  PRIMARY KEY (tombstone_id)
);

CREATE UNIQUE INDEX tombstones_uq_fediverse_uri_idx ON tombstones(fediverse_uri);

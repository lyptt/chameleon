CREATE TABLE orbits (
  orbit_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  name varchar(256) NOT NULL,
  description_md text NOT NULL,
  description_html text NOT NULL,
  avatar_url varchar(2048),
  banner_url varchar(2048),
  uri varchar(2048) NOT NULL,
  is_external bool NOT NULL DEFAULT true,
  PRIMARY KEY (orbit_id)
);

CREATE INDEX orbits_post_idx ON orbits(post_id);
CREATE UNIQUE INDEX orbits_uq_name_uri_idx ON orbits(name, uri);

CREATE TABLE orbit_moderators (
  orbit_moderator_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  is_owner bool NOT NULL,
  orbit_id UUID NOT NULL,
  user_id UUID NOT NULL,
  CONSTRAINT orbit_moderators_orbit_id_fkey FOREIGN KEY (orbit_id) REFERENCES orbits(orbit_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT orbit_moderators_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE UNIQUE INDEX orbit_moderators_uq_user_orbit_idx ON orbit_moderators(orbit_id, user_id);

CREATE TABLE user_orbits (
  user_orbit_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  orbit_id UUID NOT NULL,
  user_id UUID NOT NULL,
  CONSTRAINT user_orbits_orbit_id_fkey FOREIGN KEY (orbit_id) REFERENCES orbits(orbit_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT user_orbits_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE UNIQUE INDEX user_orbits_uq_user_orbit_idx ON user_orbits(orbit_id, user_id);

ALTER TABLE posts ADD COLUMN orbit_id UUID NULL;
ALTER TABLE posts ADD CONSTRAINT posts_orbit_id_fkey FOREIGN KEY (orbit_id) REFERENCES orbits(orbit_id) ON DELETE CASCADE ON UPDATE CASCADE;

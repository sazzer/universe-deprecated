CREATE TABLE worlds(
  world_id UUID PRIMARY KEY,
  version UUID NOT NULL,
  created TIMESTAMP WITH TIME ZONE NOT NULL,
  updated TIMESTAMP WITH TIME ZONE NOT NULL,
  owner_id UUID NOT NULL REFERENCES USERS (user_id) ON DELETE RESTRICT ON UPDATE CASCADE,
  name TEXT NOT NULL,
  slug TEXT NOT NULL,
  description TEXT NOT NULL
);
CREATE UNIQUE INDEX worlds_owner_slug_key ON worlds (owner_id, UPPER(slug));
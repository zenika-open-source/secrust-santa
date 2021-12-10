CREATE TABLE sessions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL
);

CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  session_id UUID NOT NULL REFERENCES sessions ON DELETE CASCADE
);

CREATE TABLE blacklisted ( -- Many-to-many relationship
  session_id UUID NOT NULL REFERENCES sessions ON DELETE CASCADE,
  user1_id UUID NOT NULL REFERENCES users ON DELETE CASCADE,
  user2_id UUID NOT NULL REFERENCES users ON DELETE CASCADE,
  PRIMARY KEY (user1_id, user2_id)
);

CREATE TABLE tossed (
  session_id UUID NOT NULL REFERENCES sessions ON DELETE CASCADE,
  user1_id UUID NOT NULL REFERENCES users ON DELETE CASCADE,
  user2_id UUID NOT NULL REFERENCES users ON DELETE CASCADE,
  PRIMARY KEY (user1_id, user2_id)
);

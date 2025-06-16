-- Enable extension loading
-- PRAGMA enable_load_extension = 1;

-- Load the spellfix1 extension
-- SELECT load_extension('extensions/spellfix1');

CREATE TABLE IF NOT EXISTS shortcut (
  id        INTEGER UNIQUE,
  created   INTEGER NOT NULL,
  updated   INTEGER NOT NULL,
  keyword   TEXT  NOT NULL  UNIQUE,
  url       TEXT  NOT NULL,
  PRIMARY KEY(id ASC)
);
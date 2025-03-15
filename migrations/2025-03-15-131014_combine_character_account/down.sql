

-- 1. remove password password column from `characters` table
ALTER TABLE characters DROP COLUMN password;

-- 2. change `username` on `characters` table to `name`
ALTER TABLE characters RENAME COLUMN username to name; 

-- 3. add `accounts` table
CREATE TABLE accounts (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL,
  password TEXT NOT NULL,
  created TIMESTAMPTZ NOT NULL default current_timestamp
);

-- 4. add `account_id` column to `characters` table
ALTER TABLE characters ADD account_id INTEGER NOT NULL DEFAULT 0;
ALTER TABLE characters ADD CONSTRAINT characters_account_id_fkey FOREIGN KEY (account_id) REFERENCES accounts(id);

-- 1. add password to `characters` table
ALTER TABLE characters ADD password TEXT NOT NULL;

-- 2. change `name` on `characters` table to `username`
ALTER TABLE characters RENAME COLUMN name to username; 

-- 3. remove `account_id` column on `characters` table
ALTER TABLE characters DROP COLUMN account_id; 

-- 4. remove `accounts` table
DROP TABLE accounts;
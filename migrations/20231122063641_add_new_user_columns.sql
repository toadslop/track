-- Add migration script here
ALTER TABLE user_
ADD COLUMN nickname VARCHAR (50),
ADD COLUMN comment VARCHAR (200);
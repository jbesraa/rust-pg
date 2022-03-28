CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  user_info VARCHAR NOT NULL,
  username VARCHAR NOT NULL UNIQUE,
  wallet_address VARCHAR NOT NULL UNIQUE,
  social_networks JSONB NOT NULL
)

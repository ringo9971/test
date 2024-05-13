-- Your SQL goes here

CREATE TABLE users ( 
  id SERIAL PRIMARY KEY,
  timestamp TIMESTAMP NOT NULL, 
  name VARCHAR(128) NOT NULL
)

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS
  users (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    email VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(200) NOT NULL,
    firstname VARCHAR(50) NOT NULL,
    lastname VARCHAR(50) NOT NULL
  );

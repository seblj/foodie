CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DO $do$
BEGIN
   IF EXISTS (
      SELECT FROM pg_catalog.pg_roles
      WHERE  rolname = 'foodie') THEN

      RAISE NOTICE 'Role "foodie" already exists. Skipping.';
   ELSE
      BEGIN   -- nested block
         CREATE ROLE foodie LOGIN PASSWORD 'foobar';
      EXCEPTION
         WHEN duplicate_object THEN
            RAISE NOTICE 'Role "foodie" was just created by a concurrent transaction. Skipping.';
      END;
   END IF;
END
$do$;

GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA PUBLIC TO foodie;

CREATE TABLE IF NOT EXISTS
  users (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    email VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(150) NOT NULL
  );

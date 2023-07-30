CREATE TABLE IF NOT EXISTS
  recipes (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    user_id uuid NOT NULL references users (id),
    name VARCHAR(128) NOT NULL,
    description TEXT,
    instructions TEXT,
    img VARCHAR(500),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
  );

CREATE
OR REPLACE FUNCTION trigger_set_timestamp () RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp BEFORE
UPDATE ON recipes FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp ();

CREATE TABLE IF NOT EXISTS
  ingredients (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    name VARCHAR(100) UNIQUE NOT NULL
  );

DROP TYPE IF EXISTS unit;

CREATE TYPE unit AS ENUM(
  'milligram',
  'gram',
  'hectogram',
  'kilogram',
  'milliliter',
  'deciliter',
  'liter',
  'teaspoon',
  'tablespoon',
  'cup',
  'clove',
  'pinch'
);

CREATE TABLE IF NOT EXISTS
  recipe_ingredients (
    recipe_id uuid NOT NULL references recipes (id) ON DELETE CASCADE,
    ingredient_id uuid NOT NULL references ingredients (id),
    unit unit,
    amount DECIMAL
  );

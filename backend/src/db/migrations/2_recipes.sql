CREATE TABLE IF NOT EXISTS
  recipes (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    user_id uuid references users (id),
    name VARCHAR(128) NOT NULL,
    description TEXT,
    instructions TEXT,
    img VARCHAR(500)
  );

CREATE TABLE IF NOT EXISTS
  ingredients (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    name VARCHAR(100) NOT NULL
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
    ingredient_id uuid references ingredients (id),
    unit unit,
    amount INTEGER
  );

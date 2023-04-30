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
  ingredient (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    name VARCHAR(100) NOT NULL
  );

DROP TYPE IF EXISTS unit;

CREATE TYPE unit AS ENUM(
  'mg',
  'g',
  'hg',
  'kg',
  'ml',
  'dl',
  'l',
  'tsp',
  'tbsp',
  'cup'
);

CREATE TABLE IF NOT EXISTS
  recipe_ingredient (
    recipe_id uuid references recipes (id),
    ingredient_id uuid references ingredient (id),
    unit unit,
    amount INTEGER
  );

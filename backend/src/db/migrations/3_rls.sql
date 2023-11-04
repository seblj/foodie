-- RLS
CREATE FUNCTION current_user_id () RETURNS UUID AS $$ 
    SELECT
        current_setting(
            'foodie.user_id',
            false
        )::uuid 
$$ LANGUAGE SQL;

COMMENT ON FUNCTION current_user_id IS 'These needs to be set by the application before accessing the database.';

ALTER TABLE recipes ENABLE ROW LEVEL SECURITY;

ALTER TABLE ingredients ENABLE ROW LEVEL SECURITY;

-- Policies
-- Both the owner and the one the owner is sharing with can see the recipe
CREATE POLICY rls ON recipes USING (
  user_id = current_user_id ()
  OR current_user_id () in (
    SELECT
      guest_id
    FROM
      shared_recipes
  )
);

CREATE POLICY rls ON ingredients USING (user_id = current_user_id ());

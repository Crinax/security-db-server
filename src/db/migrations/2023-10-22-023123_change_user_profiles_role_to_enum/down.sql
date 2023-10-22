-- This file should undo anything in `up.sql`
ALTER TABLE user_profiles ALTER COLUMN role
  TYPE SMALLINT
  USING CASE role
    WHEN 'user'::user_profiles_roles THEN 1
    WHEN 'employee'::user_profiles_roles THEN 2
    WHEN 'law'::user_profiles_roles THEN 3
    WHEN 'admin'::user_profiles_roles THEN 4
    ELSE 1
  END;

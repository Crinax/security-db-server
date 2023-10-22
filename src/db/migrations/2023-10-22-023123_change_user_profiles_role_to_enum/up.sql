-- Your SQL goes here
ALTER TABLE user_profiles ALTER COLUMN role
  TYPE user_profiles_roles
  USING CASE role
    WHEN 1 THEN 'user'::user_profiles_roles
    WHEN 2 THEN 'employee'::user_profiles_roles
    WHEN 3 THEN 'law'::user_profiles_roles
    WHEN 4 THEN 'admin'::user_profiles_roles
    ELSE 'user'::user_profiles_roles
  END;

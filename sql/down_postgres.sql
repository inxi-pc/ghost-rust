-- users
DROP TABLE IF EXISTS ghost.users CASCADE;

-- apps
DROP TABLE IF EXISTS ghost.apps CASCADE;

-- app_fields
DROP TABLE IF EXISTS ghost.app_fields CASCADE;

-- app_settings
DROP TABLE IF EXISTS ghost.app_settings CASCADE;

-- brute
DROP TABLE IF EXISTS ghost.brute CASCADE;

-- clients
DROP TABLE IF EXISTS ghost.clients CASCADE;

-- client_trusted_domains
DROP TABLE IF EXISTS ghost.client_trusted_domains CASCADE;

-- accesstokens
DROP TABLE IF EXISTS ghost.accesstokens CASCADE;

-- invites
DROP TABLE IF EXISTS ghost.invites CASCADE;

-- migrations
DROP TABLE IF EXISTS ghost.migrations CASCADE;

-- permissions
DROP TABLE IF EXISTS ghost.permissions CASCADE;

-- permissions_apps
DROP TABLE IF EXISTS ghost.permissions_apps CASCADE;

-- permissions_roles
DROP TABLE IF EXISTS ghost.permissions_roles CASCADE;

-- permissions_users
DROP TABLE IF EXISTS ghost.permissions_users CASCADE;

-- tags
DROP TABLE IF EXISTS ghost.tags CASCADE;

-- posts
DROP TABLE IF EXISTS ghost.posts CASCADE;

-- posts_tags
DROP TABLE IF EXISTS ghost.posts_tags CASCADE;

-- refreshtokens
DROP TABLE IF EXISTS ghost.refreshtokens CASCADE;

-- roles
DROP TABLE IF EXISTS ghost.roles CASCADE;

-- roles_users
DROP TABLE IF EXISTS ghost.roles_users CASCADE;

-- settings
DROP TABLE IF EXISTS ghost.settings CASCADE;

-- subscribers
DROP TABLE IF EXISTS ghost.subscribers CASCADE;
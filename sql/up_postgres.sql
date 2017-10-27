-- users
CREATE TABLE ghost.users (
  id SERIAL NOT NULL,
  name varchar(191) NOT NULL,
  slug varchar(191) NOT NULL,
  ghost_auth_access_token varchar(32) DEFAULT NULL,
  ghost_auth_id varchar(24) DEFAULT NULL,
  password varchar(60) NOT NULL,
  email varchar(191) NOT NULL,
  profile_image varchar(2000) DEFAULT NULL,
  cover_image varchar(2000) DEFAULT NULL,
  bio text,
  website varchar(2000) DEFAULT NULL,
  location text,
  facebook varchar(2000) DEFAULT NULL,
  twitter varchar(2000) DEFAULT NULL,
  accessibility text,
  status varchar(50) NOT NULL DEFAULT 'active',
  locale varchar(6) DEFAULT NULL,
  visibility varchar(50) NOT NULL DEFAULT 'public',
  meta_title varchar(2000) DEFAULT NULL,
  meta_description varchar(2000) DEFAULT NULL,
  tour text,
  last_seen TIMESTAMP DEFAULT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  created_by varchar(24) NOT NULL,
  updated_at TIMESTAMP DEFAULT NULL,
  updated_by varchar(24) DEFAULT NULL,
  PRIMARY KEY (id),
  UNIQUE (slug),
  UNIQUE (email)
);

-- apps
CREATE TABLE ghost.apps (
  id SERIAL NOT NULL,
  name varchar(191) NOT NULL,
  slug varchar(191) NOT NULL,
  version varchar(50) NOT NULL,
  status varchar(50) NOT NULL DEFAULT 'inactive',
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  created_by varchar(24) NOT NULL,
  updated_at TIMESTAMP DEFAULT NULL,
  updated_by varchar(24) DEFAULT NULL,
  PRIMARY KEY (id),
  UNIQUE (name),
  UNIQUE (slug)
);

-- app_fields
CREATE TABLE ghost.app_fields (
  id SERIAL NOT NULL,
  key varchar(50) NOT NULL,
  value text,
  type varchar(50) NOT NULL DEFAULT 'html',
  app_id int NOT NULL,
  relatable_id varchar(24) NOT NULL,
  relatable_type varchar(50) NOT NULL DEFAULT 'posts',
  active smallint NOT NULL DEFAULT '1',
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  created_by varchar(24) NOT NULL,
  updated_at TIMESTAMP DEFAULT NULL,
  updated_by varchar(24) DEFAULT NULL,
  PRIMARY KEY (id),
  CONSTRAINT app_fields_app_id_foreign FOREIGN KEY (app_id) REFERENCES ghost.apps (id)
);

-- app_settings
CREATE TABLE ghost.app_settings (
  id SERIAL NOT NULL,
  key varchar(50) NOT NULL,
  value text,
  app_id int NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  created_by varchar(24) NOT NULL,
  updated_at TIMESTAMP DEFAULT NULL,
  updated_by varchar(24) DEFAULT NULL,
  PRIMARY KEY (id),
  UNIQUE (key),
  CONSTRAINT app_settings_app_id_foreign FOREIGN KEY (app_id) REFERENCES ghost.apps (id)
);

-- brute
CREATE TABLE ghost.brute (
  key varchar(191) NOT NULL,
  firstRequest bigint NOT NULL,
  lastRequest bigint NOT NULL,
  lifetime bigint NOT NULL,
  count int NOT NULL
);

-- clients
CREATE TABLE ghost.clients (
  id SERIAL NOT NULL,
  uuid varchar(36) NOT NULL,
  name varchar(50) NOT NULL,
  slug varchar(50) NOT NULL,
  secret varchar(191) NOT NULL,
  redirection_uri varchar(2000) DEFAULT NULL,
  client_uri varchar(2000) DEFAULT NULL,
  auth_uri varchar(2000) DEFAULT NULL,
  logo varchar(2000) DEFAULT NULL,
  status varchar(50) NOT NULL DEFAULT 'development',
  type varchar(50) NOT NULL DEFAULT 'ua',
  description varchar(2000) DEFAULT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  created_by varchar(24) NOT NULL,
  updated_at TIMESTAMP DEFAULT NULL,
  updated_by varchar(24) DEFAULT NULL,
  PRIMARY KEY (id),
  UNIQUE (name),
  UNIQUE (slug)
);

-- client_trusted_domains
CREATE TABLE ghost.client_trusted_domains (
  id SERIAL NOT NULL,
  client_id int NOT NULL,
  trusted_domain varchar(2000) DEFAULT NULL,
  PRIMARY KEY (id),
  CONSTRAINT client_trusted_domains_client_id_foreign FOREIGN KEY (client_id) REFERENCES ghost.clients (id)
);

-- accesstokens
CREATE TABLE ghost.accesstokens (
  id SERIAL NOT NULL,
  token varchar(191) NOT NULL,
  user_id int NOT NULL,
  client_id int NOT NULL,
  issued_by varchar(24) DEFAULT NULL,
  expires bigint NOT NULL,
  PRIMARY KEY (id),
  UNIQUE (token),
  CONSTRAINT accesstokens_client_id_foreign FOREIGN KEY (client_id) REFERENCES ghost.clients (id),
  CONSTRAINT accesstokens_user_id_foreign FOREIGN KEY (user_id) REFERENCES ghost.users (id)
);

-- invites
CREATE TABLE invites (
  id SERIAL NOT NULL,
  role_id varchar(24) NOT NULL,
  status varchar(50) NOT NULL DEFAULT 'pending',
  token varchar(191) NOT NULL,
  email varchar(191) NOT NULL,
  expires bigint NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  created_by varchar(24) NOT NULL,
  updated_at TIMESTAMP DEFAULT NULL,
  updated_by varchar(24) DEFAULT NULL,
  PRIMARY KEY (id),
  UNIQUE (token),
  UNIQUE (email)
);

-- migrations
CREATE TABLE ghost.migrations (
  id SERIAL NOT NULL,
  name varchar(255) DEFAULT NULL,
  version varchar(255) DEFAULT NULL,
  currentVersion varchar(255) DEFAULT NULL,
  PRIMARY KEY (id)
);

-- permissions
CREATE TABLE ghost.permissions (
  id SERIAL NOT NULL,
  name varchar(50) NOT NULL,
  object_type varchar(50) NOT NULL,
  action_type varchar(50) NOT NULL,
  object_id varchar(24) DEFAULT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  created_by varchar(24) NOT NULL,
  updated_at TIMESTAMP DEFAULT NULL,
  updated_by varchar(24) DEFAULT NULL,
  PRIMARY KEY (id),
  UNIQUE (name)
);

-- permissions_apps
CREATE TABLE ghost.permissions_apps (
  id SERIAL NOT NULL,
  app_id int NOT NULL,
  permission_id varchar(24) NOT NULL,
  PRIMARY KEY (id)
);

-- permissions_roles
CREATE TABLE ghost.permissions_roles (
  id SERIAL NOT NULL,
  role_id varchar(24) NOT NULL,
  permission_id varchar(24) NOT NULL,
  PRIMARY KEY (id)
);

-- permissions_users

CREATE TABLE ghost.permissions_users (
  id SERIAL NOT NULL,
  user_id int NOT NULL,
  permission_id varchar(24) NOT NULL,
  PRIMARY KEY (id)
);

-- tags
CREATE TABLE ghost.tags (
  id SERIAL NOT NULL,
  name varchar(191) NOT NULL,
  slug varchar(191) NOT NULL,
  description text,
  feature_image varchar(2000) DEFAULT NULL,
  parent_id varchar(191) DEFAULT NULL,
  visibility varchar(50) NOT NULL DEFAULT 'public',
  meta_title varchar(2000) DEFAULT NULL,
  meta_description varchar(2000) DEFAULT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  created_by varchar(24) NOT NULL,
  updated_at TIMESTAMP DEFAULT NULL,
  updated_by varchar(24) DEFAULT NULL,
  PRIMARY KEY (id),
  UNIQUE (slug)
);

-- posts
CREATE TABLE ghost.posts (
  id SERIAL NOT NULL,
  uuid varchar(36) NOT NULL,
  title varchar(2000) NOT NULL,
  slug varchar(191) NOT NULL,
  mobiledoc text,
  html text,
  amp text,
  plaintext text,
  feature_image varchar(2000) DEFAULT NULL,
  featured smallint NOT NULL DEFAULT '0',
  page smallint NOT NULL DEFAULT '0',
  status varchar(50) NOT NULL DEFAULT 'draft',
  locale varchar(6) DEFAULT NULL,
  visibility varchar(50) NOT NULL DEFAULT 'public',
  meta_title varchar(2000) DEFAULT NULL,
  meta_description varchar(2000) DEFAULT NULL,
  author_id varchar(24) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  created_by varchar(24) NOT NULL,
  updated_at TIMESTAMP DEFAULT NULL,
  updated_by varchar(24) DEFAULT NULL,
  published_at TIMESTAMP DEFAULT NULL,
  published_by varchar(24) DEFAULT NULL,
  custom_excerpt varchar(2000) DEFAULT NULL,
  codeinjection_head text,
  codeinjection_foot text,
  og_image varchar(2000) DEFAULT NULL,
  og_title varchar(300) DEFAULT NULL,
  og_description varchar(500) DEFAULT NULL,
  twitter_image varchar(2000) DEFAULT NULL,
  twitter_title varchar(300) DEFAULT NULL,
  twitter_description varchar(500) DEFAULT NULL,
  custom_template varchar(100) DEFAULT NULL,
  PRIMARY KEY (id),
  UNIQUE (slug)
);

-- posts_tags
CREATE TABLE ghost.posts_tags (
  id SERIAL NOT NULL,
  post_id int NOT NULL,
  tag_id int NOT NULL,
  sort_order int NOT NULL DEFAULT '0',
  PRIMARY KEY (id),
  CONSTRAINT posts_tags_post_id_foreign FOREIGN KEY (post_id) REFERENCES ghost.posts (id),
  CONSTRAINT posts_tags_tag_id_foreign FOREIGN KEY (tag_id) REFERENCES ghost.tags (id)
);

-- refreshtokens
CREATE TABLE ghost.refreshtokens (
  id SERIAL NOT NULL,
  token varchar(191) NOT NULL,
  user_id int NOT NULL,
  client_id int NOT NULL,
  expires bigint NOT NULL,
  PRIMARY KEY (id),
  UNIQUE (token),
  CONSTRAINT refreshtokens_client_id_foreign FOREIGN KEY (client_id) REFERENCES ghost.clients (id),
  CONSTRAINT refreshtokens_user_id_foreign FOREIGN KEY (user_id) REFERENCES ghost.users (id)
);

-- roles
CREATE TABLE ghost.roles (
  id SERIAL NOT NULL,
  name varchar(50) NOT NULL,
  description varchar(2000) DEFAULT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  created_by varchar(24) NOT NULL,
  updated_at TIMESTAMP DEFAULT NULL,
  updated_by varchar(24) DEFAULT NULL,
  PRIMARY KEY (id),
  UNIQUE (name)
);

-- roles_users
CREATE TABLE ghost.roles_users (
  id SERIAL NOT NULL,
  role_id varchar(24) NOT NULL,
  user_id int NOT NULL,
  PRIMARY KEY (id)
);

-- settings
CREATE TABLE ghost.settings (
  id SERIAL NOT NULL,
  key varchar(50) NOT NULL,
  value text,
  type varchar(50) NOT NULL DEFAULT 'core',
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  created_by varchar(24) NOT NULL,
  updated_at TIMESTAMP DEFAULT NULL,
  updated_by varchar(24) DEFAULT NULL,
  PRIMARY KEY (id),
  UNIQUE (key)
);

-- subscribers
CREATE TABLE ghost.subscribers (
  id SERIAL NOT NULL,
  name varchar(191) DEFAULT NULL,
  email varchar(191) NOT NULL,
  status varchar(50) NOT NULL DEFAULT 'pending',
  post_id varchar(24) DEFAULT NULL,
  subscribed_url varchar(2000) DEFAULT NULL,
  subscribed_referrer varchar(2000) DEFAULT NULL,
  unsubscribed_url varchar(2000) DEFAULT NULL,
  unsubscribed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  created_by varchar(24) NOT NULL,
  updated_at TIMESTAMP DEFAULT NULL,
  updated_by varchar(24) DEFAULT NULL,
  PRIMARY KEY (id),
  UNIQUE (email)
);
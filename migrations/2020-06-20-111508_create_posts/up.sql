CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)

INSERT INTO "public"."posts"("id", "title", "body", "published") VALUES (1, '2', '2', 'f');
INSERT INTO "public"."posts"("id", "title", "body", "published") VALUES (2, '2', '2', 'f');
INSERT INTO "public"."posts"("id", "title", "body", "published") VALUES (3, '2', '2', 't');


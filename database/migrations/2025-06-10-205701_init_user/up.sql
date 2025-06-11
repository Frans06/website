-- Your SQL goes here
CREATE TABLE "users"(
	"id" UUID NOT NULL PRIMARY KEY,
	"email" VARCHAR NOT NULL,
	"name" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL
);

CREATE TABLE "posts"(
	"id" UUID NOT NULL PRIMARY KEY,
	"title" VARCHAR NOT NULL,
	"content" TEXT NOT NULL,
	"user_id" UUID NOT NULL,
	"published" BOOL NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL,
	FOREIGN KEY ("user_id") REFERENCES "users"("id")
);


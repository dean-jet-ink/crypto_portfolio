migrate:
	sqlx db create
	sqlx migrate run --source ./src/app/infrastructure/db/migrations

migrate-test:
	DATABASE_URL=$(cat .env | grep TEST_DATABASE_URL | cut -d '=' -f2) sqlx db create
	DATABASE_URL=$(cat .env | grep TEST_DATABASE_URL | cut -d '=' -f2) sqlx migrate run --source ./src/app/infrastructure/db/migrations
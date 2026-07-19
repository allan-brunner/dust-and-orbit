dev-up:
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml up -d --build
	
dev-down:
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml down
	
prod-up:
	docker compose -f docker-compose.yaml -f docker-compose.prod.yaml pull
	docker compose -f docker-compose.yaml -f docker-compose.prod.yaml up -d --build
	
prod-down:
	docker compose -f docker-compose.yaml -f docker-compose.prod.yaml down

test-prod:
	docker compose -f docker-compose.yaml -f docker-compose.prod.yaml -f docker-compose.local-prod.yaml up -d --build

test-prod-down:
	docker compose -f docker-compose.yaml -f docker-compose.prod.yaml -f docker-compose.local-prod.yaml down

include .env
prepare:
	cd server && DATABASE_URL="postgres://$(PG_LOGIN):$(PG_PASS)@$(PG_HOST):$(PG_PORT)/$(PG_DB)" cargo sqlx prepare
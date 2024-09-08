up:
	docker compose up -d

down:
	docker compose down

client:
	docker exec -it client bash

server:
	docker exec -it server bash

db:
	docker exec -it db bash
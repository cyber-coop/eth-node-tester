postgres:
	docker run --name postgres -e POSTGRES_PASSWORD=wow -e POSTGRES_DB=blockchains -p 5432:5432 -d postgres

dump:
	docker exec -i postgres psql -U postgres -d blockchains < nodes_backup.sql

build:
	docker build -t node-tester:v1 .

run:
	docker run --name node-tester --network host -v ./config.toml:/config.toml:ro node-tester:v1 
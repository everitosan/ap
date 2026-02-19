dev:
	@$(MAKE) -j2 dev/front dev/back

dev/front:
	@sleep 2
	@cd front && npm run dev

dev/back:
	@docker compose -f ./back/utils/dev/compose.yml up -d
	@cd back/apps/server && cargo run

dev/db:
	@docker exec -it ap_db psql ap_db -U evesan

build:
	@./utils/scripts/build.sh

deploy:
	@./utils/scripts/deploy.sh
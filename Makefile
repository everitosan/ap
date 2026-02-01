dev:
	@cd front && npm run dev

build:
	@cd front && npm run build

deploy:
	@cd front/apps/website/dist && 7z a ap.7z ./
	@scp front/apps/website/dist/ap.7z eve-dev:/home/ubuntu/apps
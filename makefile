build-and-run:
	cd charts-frontend && npm run build
	cp -r charts-frontend/public monitoring-backend/static
	cd monitoring-backend && cargo run

ENV := local

all: init-namespace init-ingress install

# --- k3s management ---
install-k3s:
	./scripts/install_k3s.sh

k3s-up:
	sudo systemctl start k3s

k3s-down:
	sudo systemctl stop k3s

# --- Initial setup (run once) ---
init-namespace:
	kubectl apply -f ./helm/namespace.yml

init-ingress:
	helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
	helm repo update

secrets:
	kubectl create secret tls taxes-tls-secret --cert=/etc/ssl/private/taxes-compare.com.pem --key=/etc/ssl/private/taxes-compare.com.key --namespace taxes-compare

# --- Install (first time) ---
install-backend:
	helm install backend-release ./helm/backend/ -f values/$(ENV)/backend/backend-values.yml
install-frontend:
	helm install frontend-release ./helm/browser/ -f values/$(ENV)/browser/browser-values.yml
install-ingress:
	helm install ingress-nginx ingress-nginx/ingress-nginx

# --- Upgrade (subsequent deploys) ---
upgrade-frontend:
	helm upgrade frontend-release ./helm/browser/ -f values/$(ENV)/browser/browser-values.yml
upgrade-backend:
	helm upgrade backend-release ./helm/backend/ -f values/$(ENV)/backend/backend-values.yml
upgrade-ingress:
	helm upgrade ingress-nginx ingress-nginx/ingress-nginx

# --- Deploy (install-or-upgrade, used by CI/CD) ---
deploy:
	helm upgrade --install backend-release ./helm/backend/ \
		-f values/$(ENV)/backend/backend-values.yml \
		$(if $(BACKEND_IMAGE),--set image=$(BACKEND_IMAGE)) \
		--namespace taxes-compare --wait --timeout 120s
	helm upgrade --install frontend-release ./helm/browser/ \
		-f values/$(ENV)/browser/browser-values.yml \
		$(if $(FRONTEND_IMAGE),--set image=$(FRONTEND_IMAGE)) \
		--namespace taxes-compare --wait --timeout 120s

# --- Teardown ---
uninstall:
	helm uninstall backend-release
	helm uninstall frontend-release

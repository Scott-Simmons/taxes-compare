ENV := local

all: init-namespace init-ingress install

install-k3s:
	./scripts/install_k3s.sh

k3s-up:
	sudo systemctl start k3s

k3s-down:
	sudo systemctl stop k3s

upgrade-frontend:
	helm upgrade frontend-release ./helm/browser/ -f values/$(ENV)/browser/browser-values.yml
upgrade-backend:
	helm upgrade backend-release ./helm/backend/ -f values/$(ENV)/backend/backend-values.yml
upgrade-ingress:
	helm upgrade ingress-nginx ingress-nginx/ingress-nginx

install-backend:
	helm install backend-release ./helm/backend/ -f values/$(ENV)/backend/backend-values.yml
install-frontend:
	helm install frontend-release ./helm/browser/ -f values/$(ENV)/browser/browser-values.yml
# note ingress is part of frontend release
install-ingress:
	helm install ingress-nginx ingress-nginx/ingress-nginx

uninstall:
	helm uninstall backend-release
	helm uninstall frontend-release

init-ingress:
	helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
	helm repo update

init-namespace:
	kubectl apply -f ./helm/namespace.yml

secrets:
	kubectl create secret tls taxes-tls-secret --cert=/etc/ssl/private/taxes-compare.com.pem --key=/etc/ssl/private/taxes-compare.com.key --namespace taxes-compare

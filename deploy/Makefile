# Reccomended to use K3s for local development
ENV := local
upgrade:
	helm upgrade backend-release ./helm/backend/ -f values/$(ENV)/backend/backend-values.yml
	helm upgrade frontend-release ./helm/browser/ -f values/$(ENV)/browser/browser-values.yml

install:
	helm install backend-release ./helm/backend/ -f values/local/backend/backend-values.yml
	helm install frontend-release ./helm/browser/ -f values/local/browser/browser-values.yml

init-ingress:
	helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
	helm repo update
	helm install ingress-nginx ingress-nginx/ingress-nginx

upgrade-ingress:
	helm upgrade ingress-nginx ingress-nginx/ingress-nginx

init-namespace:
	kubectl apply -f ./helm/namespace.yml

uninstall:
	helm uninstall backend-release
	helm uninstall frontend-release

secrets:
	kubectl create secret tls taxes-tls-secret --cert=/etc/nginx/ssl/taxes-compare.com.crt --key=/etc/nginx/ssl/taxes-compare.com.key --namespace taxes-compare

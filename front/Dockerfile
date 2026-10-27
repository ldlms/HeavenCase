# Étape 1 : Build de l'application React
FROM node:22-alpine AS build

WORKDIR /app

# Mise en cache optimale des dépendances
COPY package.json package-lock.json ./
RUN npm ci

# Copie du code source et compilation
COPY . .
RUN npm run build

# Étape 2 : Image d'exécution légère Nginx
FROM nginx:alpine

# Configuration Nginx avec redirection SPA
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copie des fichiers compilés depuis l'étape de build
COPY --from=build /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

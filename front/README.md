# Heaven Case - Front-end

Application front-end React 19 pour la plateforme Heaven Case, propulsée par Vite et stylisée avec Tailwind CSS v4.

## Organisation du projet

```
src/
├── assets/          # Fichiers statiques et médias
├── components/      # Composants UI réutilisables (ex: Header)
├── pages/           # Vues et pages de l'application (ex: HomePage)
├── App.tsx          # Composant racine
├── index.css        # Point d'entrée CSS (import Tailwind CSS v4)
└── main.tsx         # Point de montage de l'application
```

## Commandes disponibles

```bash
# Installer les dépendances
npm install

# Démarrer le serveur de développement
npm run dev

# Vérifier le code avec ESLint
npm run lint

# Compiler l'application pour la production
npm run build
```

## Docker

L'application utilise un build multi-stage (Node 22 pour le build, Nginx Alpine pour servir les fichiers statiques de la SPA).

```bash
# Construire l'image Docker
docker build -t heaven-case-front .

# Lancer le conteneur sur le port 80 (accessible sur http://localhost:8080)
docker run -d -p 8080:80 --name heaven-front heaven-case-front
```

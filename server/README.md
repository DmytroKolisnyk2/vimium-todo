# 🚀 Getting started

### Fill .env file as in .env.example

```
HOST=0.0.0.0
PORT=1337
APP_KEYS="toBeModified1,toBeModified2"
API_TOKEN_SALT=tobemodified
ADMIN_JWT_SECRET=tobemodified
TRANSFER_TOKEN_SALT=tobemodified
JWT_SECRET=tobemodified
# Database
DATABASE_CLIENT=tobemodified
DATABASE_HOST=tobemodified
DATABASE_PORT=tobemodified
DATABASE_NAME=tobemodified
DATABASE_USERNAME=tobemodified
DATABASE_PASSWORD=tobemodified
DATABASE_SSL=tobemodified
JWT_SECRET=tobemodified
```

### Run PostgreSQL db using Docker

```
docker-compose up -d
```

### `develop`


```
npm run develop
# or
yarn develop
```

### `start`

```
npm run start
# or
yarn start
```

### `build`


```
npm run build
# or
yarn build
```

## ⚙️ Deployment

```
yarn strapi deploy
```


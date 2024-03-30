# App Nuxt

## Nuxt 3 Minimal Starter

Look at the [Nuxt 3 documentation](https://nuxt.com/docs/getting-started/introduction) to learn more.

## Setup

Make sure to install the dependencies:

```bash
# npm
npm install

# pnpm
pnpm install

# yarn
yarn install

# bun
bun install
```

## Development Server

Start the development server on `http://localhost:3000`:

```bash
# npm
npm run dev

# pnpm
pnpm run dev

# yarn
yarn dev

# bun
bun run dev
```

## Production

Build the application for production:

```bash
# npm
npm run build

# pnpm
pnpm run build

# yarn
yarn build

# bun
bun run build
```

Locally preview production build:

```bash
# npm
npm run preview

# pnpm
pnpm run preview

# yarn
yarn preview

# bun
bun run preview
```
## Config dontenv .env file environment variables
Loads environment variables from .env

```shell
cp .env-example .env # run execute in terminal

# edit file .env
NUXT_BASE_URL=https://fakestoreapi.com
NUXT_PUBLIC_API_RUST_BASE_URL=https://jsonplaceholder.typicode.com/
NUXT_HOST=0.0.0.0
NUXT_PORT=3000

```

Check out the [deployment documentation](https://nuxt.com/docs/getting-started/deployment) for more information.

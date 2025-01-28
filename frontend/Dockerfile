ARG NODE_VERSION=23

FROM node:${NODE_VERSION}-slim AS base
ARG PORT=3000
WORKDIR /app

FROM base AS build
COPY --link package.json package-lock.json ./
RUN npm ci
COPY --link . .
RUN npm run build

FROM base
ENV PORT=$PORT
ENV NODE_ENV=production
COPY --from=build /app/.output /app/.output
CMD [ "node", ".output/server/index.mjs" ]
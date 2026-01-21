# ==============================================================================
# 1. DEPENDENCIES STAGE
#    Install dependencies only when package.json changes.
# ==============================================================================
FROM node:18-alpine AS deps
WORKDIR /app
# Check https://github.com/nodejs/docker-node/tree/b4117f9333da4138b03a546ec926ef50a31506c3#nodealpine to understand why libc6-compat might be needed.
RUN apk add --no-cache libc6-compat
COPY package.json package-lock.json ./
RUN npm ci

# ==============================================================================
# 2. BUILD STAGE
#    Compile the Next.js application.
# ==============================================================================
FROM node:18-alpine AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .

# Build the project
RUN npm run build

# ==============================================================================
# 3. RUNTIME STAGE
#    Run the application in production mode.
# ==============================================================================
FROM node:18-alpine AS runner
WORKDIR /app

ENV NODE_ENV production

# Don't run as root user for security
RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs

# Copy necessary files for running the app
COPY --from=builder /app/public ./public

# Next.js automatically creates a standalone folder that isolates just what is needed
# This reduces the image size significantly.
# NOTE: You must enable "output: 'standalone'" in next.config.ts (we will check this next)
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

USER nextjs

EXPOSE 3000

ENV PORT 3000
# set hostname to localhost
ENV HOSTNAME "0.0.0.0"

# server.js is created automatically by the standalone build
CMD ["node", "server.js"]

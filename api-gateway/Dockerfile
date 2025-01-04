# ===== Etapa de construcción del Backend =====
FROM rust:1.83.0 AS builder-backend
WORKDIR /usr/src/app

# Instalar dependencias necesarias
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copiar archivos de manifiesto y construir dependencias
COPY backend-api-gateway/Cargo.toml backend-api-gateway/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copiar el código fuente real y construir la aplicación
RUN rm -f src/main.rs
COPY backend-api-gateway/src/ src/
RUN cargo build --release

# ===== Etapa de construcción del Frontend =====
FROM node:23.5-alpine AS builder-frontend
WORKDIR /app

# Definir argumentos de build
ARG API_URL=http://localhost:3000
ARG DEBUG_MODE=true
ARG APP_VERSION=1.0.0

# Establecer variables de entorno para Vite
ENV VITE_API_URL=$API_URL
ENV VITE_DEBUG_MODE=$DEBUG_MODE
ENV VITE_APP_VERSION=$APP_VERSION

# Copiar y instalar dependencias
COPY frontend-api-gateway/package*.json ./
RUN npm install

# Copiar el código fuente y construir la aplicación
COPY frontend-api-gateway/. .
RUN npm run build  # Eliminado '|| true' para detectar errores durante la construcción

# ===== Etapa de Runtime =====
FROM debian:bookworm-slim
WORKDIR /app

# Instalar dependencias de runtime, Nginx y curl
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 nginx curl && \
    rm -rf /var/lib/apt/lists/*

# Copiar el binario del backend
COPY --from=builder-backend /usr/src/app/target/release/api-gateway /app/api-gateway

# Copiar los archivos del frontend a Nginx
COPY --from=builder-frontend /app/dist /usr/share/nginx/html

# Eliminar todas las configuraciones de Nginx en conf.d para evitar conflictos
RUN rm -f /etc/nginx/conf.d/*.conf

# Copiar el archivo nginx.conf desde frontend-api-gateway
COPY frontend-api-gateway/nginx.conf /etc/nginx/conf.d/default.conf

# **Eliminar todas las configuraciones en sites-enabled para evitar conflictos**
RUN rm -f /etc/nginx/sites-enabled/*

# Crear usuario no root para el backend
RUN useradd -m -U -s /bin/false api-gateway && \
    chown -R api-gateway:api-gateway /app

# Crear los directorios necesarios para Nginx y asignar permisos
RUN mkdir -p /var/lib/nginx/body /var/lib/nginx/proxy /var/lib/nginx/fastcgi /var/log/nginx && \
    chown -R api-gateway:api-gateway /var/lib/nginx /var/log/nginx

# Asignar propiedad al directorio root de Nginx
RUN chown -R api-gateway:api-gateway /usr/share/nginx/html

# Eliminar la directiva 'user' en nginx.conf
RUN sed -i '/^user /d' /etc/nginx/nginx.conf

# Reemplazar la directiva 'pid' en el archivo principal de Nginx
RUN sed -i 's|^pid .*;|pid /var/lib/nginx/nginx.pid;|' /etc/nginx/nginx.conf

# Copiar script de inicio
COPY start.sh /start.sh
RUN chmod +x /start.sh

# Exponer puertos
EXPOSE 80 3000

# Definir usuario
USER api-gateway

# Comando de inicio
CMD ["/start.sh"]


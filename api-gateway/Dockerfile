
root@elnidodelaguila:/home/torrefacto/flusso/api-gateway# cat Dockerfile
#################################################################
# 1. Stage de compilación del FRONTEND
#################################################################
FROM node:23.5.0 AS frontend-builder

# Actualizar los paquetes del sistema
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Cambia el WORKDIR a algo descriptivo
WORKDIR /frontend

# Copiar manifiestos de npm
COPY frontend-api-gateway/package*.json ./

# Instalar dependencias
RUN npm install

# Copiar el resto del código del frontend
COPY frontend-api-gateway/ ./

# Compilar el frontend
RUN npm run build
# => genera la carpeta /frontend/dist (o /frontend/build, según tu config)

#################################################################
# 2. Stage de compilación del BACKEND (Rust)
#################################################################
FROM rust:1.83 AS backend-builder

WORKDIR /backend

# Actualizar y actualizar paquetes del sistema e Instalar paquetes del sistema que Rust pudiera necesitar (ej. openssl)
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y pkg-config libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copiamos los archivos Cargo.toml y Cargo.lock
COPY backend-api-gateway/Cargo.toml .
COPY backend-api-gateway/Cargo.lock .

# Crear src vacía para precachear dependencias
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
# Esto compila las dependencias y las deja cacheadas

# Borrar el dummy main.rs
RUN rm -f src/main.rs

# Copiar el código real del backend
COPY backend-api-gateway/src/ src/

# Compilar el backend
RUN cargo build --release
# El binario resultante quedará en /backend/target/release/api-gateway (ajusta nombre si es distinto)

#################################################################
# 3. Imagen final (runtime)
#################################################################
FROM debian:stable-slim

# Crear carpeta /app y usarla como directorio de trabajo
WORKDIR /app

# Copiar el binario Rust
COPY --from=backend-builder /backend/target/release/api-gateway /app/api-gateway

# Copiar la carpeta build del frontend
# (asumiendo que Vite genera /frontend/dist)
COPY --from=frontend-builder /frontend/dist /app/build
# O si tu build se llama `build` en vez de `dist`, ajusta la ruta:
# COPY --from=frontend-builder /frontend/build /app/build

# (Opcional) Crear usuario no root
RUN useradd -m -U -s /bin/false api-gateway && \
    chown -R api-gateway:api-gateway /app
USER api-gateway

# Exponer el puerto 3000 (donde corre Actix, por ejemplo)
EXPOSE 3000

# Arrancar el binario
CMD ["./api-gateway"]


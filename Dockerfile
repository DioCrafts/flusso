# Etapa de construcción
FROM rust:latest as builder
WORKDIR /app

# Instala musl-tools para permitir la compilación estática
RUN apt-get update && \
    apt-get install -y --no-install-recommends musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    rm -rf /var/lib/apt/lists/*

# Copia los archivos de configuración de Rust y descarga las dependencias primero para aprovechar el cacheo
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked

# Copia el código fuente y construye el proyecto en modo release con target estático
COPY src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl

# Etapa final
FROM debian:bookworm-slim
WORKDIR /usr/local/bin

# Copia el binario estático desde la etapa de construcción
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/flusso .

# Copia los archivos estáticos
COPY src/gui/static /usr/local/bin/static

# Exponer los puertos en los que la aplicación escucha
EXPOSE 8080
EXPOSE 8081

# Ejecuta la aplicación
CMD ["flusso"]


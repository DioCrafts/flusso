# Etapa 1: Construcción de los archivos estáticos
FROM node:18-alpine AS builder

# Establece el directorio de trabajo
WORKDIR /app

# Copia los archivos necesarios para instalar las dependencias
COPY package.json package-lock.json ./

# Instala las dependencias del proyecto
RUN npm install

# Copia el código fuente al contenedor
COPY . .

# Construye los archivos estáticos con Vite
RUN npm run build

# Etapa 2: Servidor Nginx para servir los archivos estáticos
FROM nginx:stable-alpine

# Copia los archivos estáticos generados a la carpeta predeterminada de Nginx
COPY --from=builder /app/dist /usr/share/nginx/html

# Copia la configuración personalizada de Nginx (si existe)
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expone el puerto 80 para el tráfico HTTP
EXPOSE 80

# Inicia Nginx en modo foreground
CMD ["nginx", "-g", "daemon off;"]

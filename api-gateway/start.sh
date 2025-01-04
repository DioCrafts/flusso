#!/bin/sh

# Iniciar Nginx en primer plano
nginx -g "daemon off;" &

# Iniciar el backend de la API Gateway
./api-gateway &

# Esperar a que cualquiera de los procesos termine
wait %1 || exit $?  # Espera el primer proceso en background (Nginx)
wait %2 || exit $?  # Espera el segundo proceso en background (Backend)


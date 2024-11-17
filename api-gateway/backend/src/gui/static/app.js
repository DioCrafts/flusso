document.addEventListener('DOMContentLoaded', function () {
    // Cargar Gateways dinámicamente desde el backend
    fetch('/api/gateways')
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load gateways');
            }
            return response.json();
        })
        .then(gateways => {
            const gatewayList = document.getElementById('gateway-list');
            gateways.forEach(gateway => {
                const li = document.createElement('li');
                li.textContent = gateway;  // Asumiendo que 'gateway' es un nombre de Gateway
                gatewayList.appendChild(li);
            });
        })
        .catch(error => console.error('Error loading gateways:', error));

    // Cargar Routes dinámicamente
    fetch('/api/routes')
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load routes');
            }
            return response.json();
        })
        .then(routes => {
            const routeList = document.getElementById('route-list');
            routes.forEach(route => {
                const li = document.createElement('li');
                li.textContent = route.name;  // Asumiendo que 'route.name' es el nombre de la ruta
                routeList.appendChild(li);
            });
        })
        .catch(error => console.error('Error loading routes:', error));

    // Actualizar el estado del servidor
    fetch('/api/status')
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load status');
            }
            return response.json();
        })
        .then(data => {
            const status = document.getElementById('status');
            status.textContent = data.status;  // Asumiendo que 'status' es el campo con el estado del servidor
        })
        .catch(error => console.error('Error loading server status:', error));
});

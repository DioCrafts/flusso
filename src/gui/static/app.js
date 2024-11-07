// src/gui/static/app.js

// Obtiene la lista de backends y actualiza el contenido de la lista en la GUI.
async function fetchBackends() {
    const response = await fetch('/backends');
    const backends = await response.json();

    const backendList = document.getElementById('backend-list');
    backendList.innerHTML = '';

    backends.forEach(backend => {
        const listItem = document.createElement('li');
        listItem.innerHTML = `
            <strong>${backend.address}</strong><br>
            Estado: ${backend.status}<br>
            Conexiones: ${backend.connections}
        `;
        backendList.appendChild(listItem);
    });

    updateChart(backends);
}

// Llama a la función `fetchBackends` al cargar la página y cada 5 segundos.
window.addEventListener('DOMContentLoaded', fetchBackends);
setInterval(fetchBackends, 5000);

// Actualiza el gráfico con los datos de los backends
function updateChart(backends) {
    const labels = backends.map(backend => backend.address);
    const data = backends.map(backend => backend.connections);

    chart.data.labels = labels;
    chart.data.datasets[0].data = data;
    chart.update();
}


// Inicializa el gráfico de conexiones
const ctx = document.getElementById('connectionsChart').getContext('2d');
const chart = new Chart(ctx, {
    type: 'bar',
    data: {
        labels: [], // Será actualizado con las direcciones de los backends
        datasets: [{
            label: 'Conexiones Activas',
            data: [], // Será actualizado con el número de conexiones de cada backend
            backgroundColor: 'rgba(75, 192, 192, 0.2)',
            borderColor: 'rgba(75, 192, 192, 1)',
            borderWidth: 1
        }]
    },
    options: {
        scales: {
            y: {
                beginAtZero: true,
                title: {
                    display: true,
                    text: 'Número de Conexiones'
                }
            },
            x: {
                title: {
                    display: true,
                    text: 'Backends'
                }
            }
        }
    }
});


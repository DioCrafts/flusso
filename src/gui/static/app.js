// Fetches the list of backends and updates the content on the dashboard.
async function fetchBackends() {
    const response = await fetch('/backends');
    const backends = await response.json();

    const backendList = document.getElementById('backend-list');
    backendList.innerHTML = '';
    backends.forEach(backend => {
        const listItem = document.createElement('li');
        listItem.innerHTML = `<strong>${backend.address}</strong><br>Status: ${backend.status}<br>Connections: ${backend.connections}`;
        backendList.appendChild(listItem);
    });

    updateChart(backends);
}

// Fetches the list of gateways and updates the Gateway tab on the dashboard.
async function fetchGateways() {
    const response = await fetch('/gateways');
    const gateways = await response.json();

    const gatewayList = document.getElementById('gateway-list');
    gatewayList.innerHTML = '';
    gateways.forEach(gateway => {
        const listItem = document.createElement('li');
        listItem.innerHTML = `<strong>${gateway.name}</strong><br>Status: ${gateway.status}<br>Routes: ${gateway.routes}`;
        gatewayList.appendChild(listItem);
    });
}

// Calls `fetchBackends` and `fetchGateways` on page load and every 5 seconds.
window.addEventListener('DOMContentLoaded', () => {
    fetchBackends();
    fetchGateways();
});
setInterval(() => {
    fetchBackends();
    fetchGateways();
}, 5000);

// Updates the chart with backend connection data.
function updateChart(backends) {
    const labels = backends.map(backend => backend.address);
    const data = backends.map(backend => backend.connections);

    chart.data.labels = labels;
    chart.data.datasets[0].data = data;
    chart.update();
}

// Initializes the chart for displaying active connections.
const ctx = document.getElementById('connectionsChart').getContext('2d');
const chart = new Chart(ctx, {
    type: 'bar',
    data: {
        labels: [], // Updated with backend addresses
        datasets: [{
            label: 'Active Connections',
            data: [], // Updated with the number of connections per backend
            backgroundColor: 'rgba(75, 192, 192, 0.2)',
            borderColor: 'rgba(75, 192, 192, 1)',
            borderWidth: 1
        }]
    },
    options: {
        scales: {
            y: {
                beginAtZero: true,
                title: { display: true, text: 'Number of Connections' }
            },
            x: {
                title: { display: true, text: 'Backends' }
            }
        }
    }
});

// src/gui/static/app.js

// Fetches the list of backends and updates the content on the dashboard.
async function fetchBackends() {
    const response = await fetch('/backends');
    const backends = await response.json();

    // Update the backend list in the HTML
    const backendList = document.getElementById('backend-list');
    backendList.innerHTML = '';

    backends.forEach(backend => {
        const listItem = document.createElement('li');
        listItem.innerHTML = `
            <strong>${backend.address}</strong><br>
            Status: ${backend.status}<br>
            Connections: ${backend.connections}
        `;
        backendList.appendChild(listItem);
    });

    updateChart(backends);
}

// Calls `fetchBackends` on page load and every 5 seconds.
window.addEventListener('DOMContentLoaded', fetchBackends);
setInterval(fetchBackends, 5000);

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
                title: {
                    display: true,
                    text: 'Number of Connections'
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

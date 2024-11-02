// src/gui/static/app.js

// Obtiene la lista de backends y actualiza el contenido de la lista en la GUI.
async function fetchBackends() {
    const response = await fetch('/backends');
    const backends = await response.json();

    const backendList = document.getElementById('backend-list');
    backendList.innerHTML = '';

    backends.forEach(backend => {
        const listItem = document.createElement('li');
        listItem.textContent = `${backend.address} - ${backend.status}`;
        backendList.appendChild(listItem);
    });
}

// Llama a la función `fetchBackends` al cargar la página.
window.addEventListener('DOMContentLoaded', fetchBackends);

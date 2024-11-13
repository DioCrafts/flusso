
# Flusso Project Documentation

## Overview

**Flusso** is an Ingress controller written in Rust that manages HTTP requests in a Kubernetes cluster, distributing them among backend services via a load balancer. This project includes a graphical user interface (GUI) for monitoring and managing backend services.

Flusso listens for changes in Kubernetes resources, such as the creation or deletion of Ingress rules, and dynamically updates available backends. It uses asynchronous and concurrent `tokio` functionalities for optimized performance.

### Key Components

1. **Event Listener**: Listens for Kubernetes Ingress events and updates the list of backends.
2. **Ingress Processor**: Processes Ingress events to update the load balancer.
3. **Load Balancer**: Distributes HTTP requests among backends using a round-robin algorithm.
4. **HTTP Proxy**: Forwards HTTP requests to the backend selected by the load balancer.
5. **Cache**: Temporarily stores HTTP responses to improve performance.
6. **Router**: Manages routes and assigns backends to specific routes for incoming requests.

---

## Code Structure

### 1. `main.rs` - Application Entry Point

This file initializes the configuration, load balancer, and services of Flusso, then runs both the Ingress controller and the GUI server concurrently.

- **CryptoProvider Setup**: Sets the default cryptographic provider at the process level using `rustls::crypto::aws_lc_rs::default_provider()`.
- **Configuration Loading**: Loads application configuration from `Settings`.
- **Load Balancer**: Initializes a load balancer (`LoadBalancer`) to distribute HTTP requests across backends.
- **Ingress Controller and GUI Server**: Uses asynchronous tasks to run both components in parallel with `tokio::try_join!`.

**Execution Flow in `main.rs`:**

1. Initializes the cryptographic provider.
2. Loads the configuration.
3. Creates the shared load balancer.
4. Configures the GUI port.
5. Runs the Ingress controller and GUI server concurrently.

---

### 2. `config/settings.rs` - Application Configuration

`Settings` loads and manages the application configuration. Main parameters include ports, configuration paths, and any other customizable settings through files or environment variables.

### 3. `proxy/load_balancer.rs` - Load Balancer

The load balancer (`LoadBalancer`) manages a list of backends and selects one of them to distribute incoming requests using the round-robin algorithm.

- **Main Methods**:
  - `new()`: Initializes the balancer with a list of backend addresses.
  - `select_backend()`: Selects the next backend in the list using round-robin.
  - `add_backend()`: Adds a backend if it is not already in the list.
  - `remove_backend()`: Removes a backend from the list.
  - `get_backends()`: Returns a list of current backends.

---

### 4. `proxy/http.rs` - HTTP Proxy

The HTTP proxy (`HttpProxy`) is responsible for forwarding requests to the backends selected by the load balancer.

- **Main Methods**:
  - `new()`: Creates an instance of the HTTP proxy with a load balancer.
  - `forward_request()`: Forwards an HTTP request to a backend selected by the balancer.
    - Receives `path`, `method`, `headers`, and `body` parameters.
    - Constructs the backend URL and sends the request.
    - Logs detailed request and response information.

---

### 5. `proxy/cache.rs` - Cache

The cache temporarily stores HTTP responses to reduce latency on repeated requests and improve performance.

- **Data Structure**:
  - `CacheEntry`: Contains the response in bytes and a timestamp for expiration control.
  - `Cache`: Uses a `HashMap` to store `CacheEntry` items with a defined TTL.

- **Main Methods**:
  - `new()`: Creates a cache instance with a specified TTL.
  - `store()`: Stores a response in the cache.
  - `retrieve()`: Retrieves a response from the cache if it has not expired.
  - `clean_expired()`: Cleans up expired entries.

---

### 6. `proxy/router.rs` - Router

The router (`Router`) assigns routes to specific backends, allowing management of which requests go to which servers.

- **Data Structure**:
  - `Route`: Defines a route with a `path` and its associated backend.

- **Main Methods**:
  - `new()`: Creates an empty router.
  - `add_route()`: Adds a new route to the router.
  - `get_backend()`: Looks up the backend associated with a given route.

---

### 7. `ingress_controller/event_listener.rs` - Ingress Event Listener

The `EventListener` listens for Kubernetes Ingress events and updates the load balancer in response to additions or deletions of Ingress resources.

- **Main Methods**:
  - `new()`: Creates an `EventListener` instance and returns it with an `IngressEvent` channel to process changes.
  - `start_listening()`: Starts listening for changes in Ingress resources and calls `process_ingress` or `remove_ingress` based on the event type.
  - `process_ingress()`: Adds the backend associated with the Ingress to the load balancer if it meets certain criteria (specific annotations).
  - `remove_ingress()`: Removes the backend associated with an Ingress from the load balancer.
  - `resolve_service_ip()`: Resolves the IP address of a service in Kubernetes.

**Execution Flow in `start_listening()`**:

1. Loads existing Ingress resources at startup and processes each of them.
2. Continuously listens for Ingress events and updates the balancer based on the event.

---

### 8. `ingress_controller/ingress_processor.rs` - Ingress Processor

The `IngressProcessor` processes `IngressEvent` events generated by the `EventListener` and updates the load balancer accordingly.

- **Data Structure**:
  - `IngressEvent`: Enumerates `Add` and `Remove` events with the backend address.

- **Main Methods**:
  - `new()`: Creates an `IngressProcessor` with a load balancer and event receiver.
  - `process_events()`: Listens for `IngressEvent`s and calls `add_backend` or `remove_backend` on the load balancer.

**Execution Flow in `process_events()`**:

1. Receives an `IngressEvent` from the channel.
2. Based on the event type (`Add` or `Remove`), adds or removes the backend from the load balancer.

---

### 9. `gui/gui_server.rs` - GUI Server (Undocumented)

This server provides a graphical interface to interact with and monitor the load balancer in real-time.

---

## Full Application Workflow

1. **Initialization (`main.rs`)**:
   - Sets up the cryptographic provider and loads the configuration.
   - Initializes the load balancer and GUI port.
   - Runs `EventListener` and `IngressProcessor` in parallel with the GUI server.

2. **Ingress Event Listening (`EventListener`)**:
   - Detects and processes events from Kubernetes Ingress resources.
   - Adds or removes backends from the load balancer based on the event type.

3. **Ingress Event Processing (`IngressProcessor`)**:
   - Receives `IngressEvent`s and updates the load balancer.

4. **Request Distribution (`HttpProxy`)**:
   - Receives HTTP requests and uses the load balancer to select a backend.
   - Sends the request to the backend and returns the response to the client.

5. **Caching (`Cache`)**:
   - Stores HTTP responses to optimize repeated request speed.

6. **Routing (`Router`)**:
   - Assigns routes to specific backends and redirects requests based on their path.

7. **GUI Interface**:
   - Allows monitoring and management of backend services in real-time.

---

## Design Considerations

- **Asynchronicity and Concurrency**: Uses `tokio` to handle asynchronous network operations and concurrency, optimizing latency and resource usage.
- **Error Handling**: Detailed error handling in each module improves system robustness.
- **Scalability**: The load balancer can manage multiple backends, adding or removing them dynamically based on Ingress events.
- **Modularity**: The application is divided into modules for easier maintenance and extensibility.

This documentation provides a thorough understanding of Flusso’s workflow and components, enabling other developers to grasp how each module interacts and contributes to the complete system.

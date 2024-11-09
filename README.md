
# Flusso - High-Performance Kubernetes Ingress Controller in Rust

<div align="center">
  
  [![Build Status](https://img.shields.io/github/actions/workflow/status/diocrafts/flusso/build-and-test.yml?branch=main)](https://github.com/diocrafts/flusso/actions)
  [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
  [![Docker Pulls](https://img.shields.io/docker/pulls/diocrafts/flusso-ingress-controller)](https://hub.docker.com/r/diocrafts/flusso-ingress-controller)
  [![Latest Release](https://img.shields.io/github/release/diocrafts/flusso.svg)](https://github.com/diocrafts/flusso/releases)
  [![GitHub Stars](https://img.shields.io/github/stars/diocrafts/flusso?style=social)](https://github.com/diocrafts/flusso/stargazers)

</div>



Flusso is an open-source, high-performance Ingress Controller for Kubernetes, built in **Rust**. Designed to provide a **lightweight**, **secure**, and **scalable** alternative to popular ingress solutions like Traefik and NGINX, Flusso delivers optimized performance and efficient load balancing tailored for cloud-native environments.

## Features

- **Lightweight and Fast**: Written in Rust, offering high performance and low memory and CPU consumption.
- **Advanced Load Balancing**: Flusso supports custom load balancing algorithms for optimized traffic distribution.
- **Secure by Design**: Implements modern TLS protocols with Rustls for enhanced security.
- **Dynamic Backends**: Automatically updates routing based on Kubernetes service changes.
- **Flexible Configuration**: Easily configurable via YAML files or environment variables.
- **Minimal Dependencies**: Avoids unnecessary dependencies for lightweight container images.

## Table of Contents

- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Kubernetes Setup](#kubernetes-setup)
- [Contributing](#contributing)
- [License](#license)

---

## Installation

To get started with Flusso, you can use either Docker or Helm to deploy the Ingress Controller in your Kubernetes cluster.

### Docker Installation

1. Pull the Docker image:
   ```sh
   docker pull diocrafts/flusso-ingress-controller:latest
   ```

2. Run the Docker container:
   ```sh
   docker run -p 8080:8080 -p 8081:8081 -e SERVER_ADDR="0.0.0.0:8080" -e TLS_ENABLED="true" diocrafts/flusso-ingress-controller
   ```

### Kubernetes Installation with Helm

1. Add your Helm repository (if not added yet):
   ```sh
   helm repo add your-repo https://your-helm-repo.com/charts
   ```

2. Install Flusso Ingress Controller:
   ```sh
   helm install flusso-ingress your-repo/flusso-ingress
   ```

For more configuration options, refer to the [values.yaml](chart/values.yaml).

---

## Configuration

Flusso supports several configuration options, both via environment variables and Helm chart values. Here are the key parameters:

- **SERVER_ADDR**: Define the address where the Ingress Controller will listen. Default is `0.0.0.0:8080`.
- **TLS_ENABLED**: Enable or disable TLS (default is `true`).
- **TLS_CERT_PATH / TLS_KEY_PATH**: Paths to TLS certificate and key files.

### Example `values.yaml` Configuration

```yaml
replicaCount: 1

image:
  repository: diocrafts/flusso-ingress-controller
  tag: latest
  pullPolicy: IfNotPresent

serviceAccount:
  create: true
  name: flusso-ingress

service:
  type: NodePort
  port: 80
  targetPort: 8080

env:
  TLS_ENABLED: "true"
  SERVER_ADDR: "0.0.0.0:8080"
```

---

## Usage

### Basic Routing

Flusso automatically routes incoming traffic to Kubernetes services defined by Ingress resources. Add an Ingress rule to direct traffic to your applications.

### Example Ingress Resource

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: example-ingress
spec:
  ingressClassName: flusso
  rules:
    - host: example.com
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: example-service
                port:
                  number: 80
```

### Monitoring

Flusso exposes a built-in web GUI accessible at `http://<controller-ip>:8081`, providing insights into active backends and routing information.

## Kubernetes Setup

Flusso is designed to work seamlessly in Kubernetes as an Ingress Controller.

### Prerequisites

- Kubernetes version 1.19 or higher
- Helm version 3 or higher

### Deploying on Minikube

1. Enable the `metrics-server` on Minikube (optional but recommended):
   ```sh
   minikube addons enable metrics-server
   ```

2. Deploy Flusso with Helm:
   ```sh
   helm install flusso-ingress your-repo/flusso-ingress
   ```

3. Check pod and service status:
   ```sh
   kubectl get pods -A
   kubectl get svc -A
   ```

---

## Contributing

We welcome contributions to make Flusso even better! If you have suggestions for improvements, open a GitHub issue or submit a pull request. Please refer to our [Contributing Guide](CONTRIBUTING.md) for more details.

---

## License

Flusso is licensed under the [MIT License](LICENSE).

---

## Contact & Support

- **Website**: [Your Website](https://yourwebsite.com)
- **GitHub**: [GitHub Repository](https://github.com/diocrafts/flusso)
- **Docker Hub**: [Docker Hub Repository](https://hub.docker.com/r/diocrafts/flusso)

For further support, reach out via GitHub issues or visit our community forums.

---

Flusso aims to be the next generation Ingress Controller for Kubernetes clusters, offering a streamlined, secure, and high-performance solution built in Rust. Join the movement and contribute today!


# 🚀 Flusso - Secure, High-Performance Kubernetes Ingress Controller in Rust 🦀🔒

**Flusso** is a powerful, secure, and high-performance Ingress Controller for Kubernetes, written in Rust. Tailored for modern cloud-native environments, Flusso offers a lightweight alternative to popular ingress solutions.

<div align="center">
  
  [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
  [![Docker Pulls](https://img.shields.io/docker/pulls/diocrafts/flusso-ingress-controller)](https://hub.docker.com/r/diocrafts/flusso-ingress-controller)
  [![Latest Release](https://img.shields.io/github/release/diocrafts/flusso.svg)](https://github.com/diocrafts/flusso/releases)
  [![GitHub Stars](https://img.shields.io/github/stars/diocrafts/flusso?style=social)](https://github.com/diocrafts/flusso/stargazers)
  [![GitHub Issues](https://img.shields.io/github/issues/diocrafts/flusso)](https://github.com/diocrafts/flusso/issues)
  [![GitHub Forks](https://img.shields.io/github/forks/diocrafts/flusso?style=social)](https://github.com/diocrafts/flusso/network/members)
  [![Last Commit](https://img.shields.io/github/last-commit/diocrafts/flusso)](https://github.com/diocrafts/flusso/commits/main)

</div>

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Kubernetes Setup](#kubernetes-setup)
- [Contributing](#contributing)
- [License](#license)
- [Contact & Support](#contact--support)

---

## Features ✨

- **Lightweight and Fast** ⚡: Built in Rust 🦀 for high performance and low memory and CPU consumption.
- **Advanced Load Balancing** 🔄: Supports custom load balancing algorithms for optimized traffic distribution.
- **Secure by Design** 🔒: Implements modern TLS protocols with Rustls for enhanced security.
- **Dynamic Backends** 🔄: Automatically updates routing based on Kubernetes service changes.
- **Flexible Configuration** 🛠️: Easily configurable via YAML files or environment variables.
- **Minimal Dependencies** 📦: Avoids unnecessary dependencies for lightweight container images.

---

## Installation

To get started with Flusso, you can use Docker or Helm to deploy the Ingress Controller in your Kubernetes cluster.

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
   helm repo add flusso-repo https://your-helm-repo.com/charts
   ```

2. Install Flusso Ingress Controller:
   ```sh
   helm install flusso-ingress flusso-repo/flusso-ingress
   ```

For more configuration options, refer to the [values.yaml](chart/values.yaml).

---

## Configuration

Flusso supports several configuration options, both via environment variables and Helm chart values.

- **SERVER_ADDR**: Define the address where the Ingress Controller will listen. Default is `0.0.0.0:8080`.
- **TLS_ENABLED**: Enable or disable TLS (default is `true`).
- **TLS_CERT_PATH / TLS_KEY_PATH**: Paths to TLS certificate and key files.

---

## Usage

Flusso automatically routes incoming traffic to Kubernetes services defined by Ingress resources.

### Monitoring

Flusso exposes a web GUI at `http://<controller-ip>:8081` with insights into backends and routing.

---

## Kubernetes Setup

Flusso is designed for seamless integration in Kubernetes.

### Prerequisites

- Kubernetes version 1.19 or higher
- Helm version 3 or higher

---

## Contributing

We welcome contributions to make Flusso even better! If you have suggestions for improvements, open a GitHub issue or submit a pull request. Please refer to our [Contributing Guide](CONTRIBUTING.md) for more details.

---

## License

Flusso is licensed under the [MIT License](LICENSE).

---

## Contact & Support

- **GitHub**: [GitHub Repository](https://github.com/diocrafts/flusso)
- **Docker Hub**: [Docker Hub Repository](https://hub.docker.com/r/diocrafts/flusso)

For further support, reach out via GitHub issues or visit our community forums.

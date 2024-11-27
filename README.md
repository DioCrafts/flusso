
<div id="top"></div>

<p align="center">Help us grow and star us on Github! ⭐️</p>

<div align="center">
  <img src="images/flusso-logo.svg" alt="Flusso Logo" width="250">
</div>

# 🚀 Flusso - Secure, High-Performance Kubernetes Ingress Controller and API Gateway in Rust 🦀🔒

**Flusso** is a secure, high-performance solution for Kubernetes, combining the functionalities of an **Ingress Controller** and an **API Gateway**. Written in **Rust**, Flusso is designed to meet the needs of modern cloud-native environments, offering a lightweight and efficient alternative to traditional solutions.


<div align="center">
  
  [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
  [![Docker Pulls](https://img.shields.io/docker/pulls/diocrafts/flusso-ingress-controller?style=for-the-badge&logo=docker)](https://hub.docker.com/r/diocrafts/flusso-ingress-controller)
  [![Latest Release](https://img.shields.io/github/release/diocrafts/flusso.svg?style=for-the-badge)](https://github.com/diocrafts/flusso/releases)
  [![GitHub Stars](https://img.shields.io/github/stars/diocrafts/flusso?style=for-the-badge&logo=github)](https://github.com/diocrafts/flusso/stargazers)
  [![GitHub Issues](https://img.shields.io/github/issues/diocrafts/flusso?style=for-the-badge)](https://github.com/diocrafts/flusso/issues)
  [![GitHub Forks](https://img.shields.io/github/forks/diocrafts/flusso?style=for-the-badge&logo=github)](https://github.com/diocrafts/flusso/network/members)
  [![Last Commit](https://img.shields.io/github/last-commit/diocrafts/flusso?style=for-the-badge)](https://github.com/diocrafts/flusso/commits/main)

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

Flusso provides two components for deployment: the **Ingress Controller** and the **API Gateway**. You can choose to deploy either or both of them depending on your needs. Both components can be deployed using **Helm** or **Docker**, and here are the instructions for each.

### 1. **Install Ingress Controller using Helm**

To install the Flusso Ingress Controller in your Kubernetes cluster, you can use Helm. Follow these steps:

1. **Add the Flusso Helm Chart repository:**

   ```bash
   helm repo add flusso https://diocrafts.github.io/flusso
   helm repo update
   ```

2. **Install the Ingress Controller:**

   ```bash
   helm install flusso-ingress-controller flusso/flusso-ingress-controller        --namespace ingress-system        --create-namespace
   ```

   This will install the Flusso Ingress Controller in the `ingress-system` namespace. You can customize your deployment by modifying values in the Helm chart.

3. **Verify the deployment:**

   Check the status of the Ingress Controller to ensure it is running:

   ```bash
   kubectl get pods -n ingress-system
   ```

---

### 2. **Install API Gateway using Helm**

To install the Flusso API Gateway in your Kubernetes cluster, follow these steps:

1. **Add the Flusso Helm Chart repository:**

   ```bash
   helm repo add flusso https://diocrafts.github.io/flusso
   helm repo update
   ```

2. **Install the API Gateway:**

   ```bash
   helm install flusso-api-gateway flusso/flusso-api-gateway        --namespace api-gateway-system        --create-namespace
   ```

   This will install the Flusso API Gateway in the `api-gateway-system` namespace. You can modify the Helm chart values to customize your deployment.

3. **Verify the deployment:**

   Check the status of the API Gateway to ensure it is running:

   ```bash
   kubectl get pods -n api-gateway-system
   ```


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

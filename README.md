# Vercel Clone

This is a clone of the popular deployment platform Vercel, built using Rust with Axum for the backend, and React for the frontend. The project aims to provide a similar experience to Vercel, allowing developers to easily deploy their web applications and static sites.

## Features

- **Deployments**: Users can create new deployments for their projects, specifying the source code repository and build settings.
- **Deployment Logs**: Real-time logs for each deployment, allowing users to monitor the build and deployment process.
- **Project Management**: Users can create and manage multiple projects, each with its own deployment history and settings.
- **Authentication**: Secure authentication system using JSON Web Tokens (JWT) or other authentication strategies.

## Technologies Used

### Backend

- **Rust**: The backend is built using the Rust programming language, known for its performance, safety, and concurrency features.
- **Axum**: A powerful web application framework for Rust, designed to be modular and easy to use.
- **AWS Services**: The backend integrates with various AWS services, including AWS Lambda for serverless functions, AWS S3 for storing built artifacts, and AWS API Gateway for exposing the API over the internet.
- **Redis**: An in-memory data store used for caching deployment metadata, logs, and other relevant information.

### Frontend

- **React**: The frontend is built using React, a popular JavaScript library for building user interfaces.
- **React Router**: A routing library for React applications, enabling navigation between different pages and views.
- **Axios** or **Fetch**: A library or built-in API for making HTTP requests to the backend API.
- **Tailwind CSS** or **Material-UI**: A utility-first CSS framework or a popular React UI library for styling components.

## Getting Started

### Prerequisites

- Rust and Cargo (Rust's package manager and build tool)
- Node.js and npm (Node.js package manager)
- AWS account and necessary access keys/credentials
- Redis server (local or remote)

### Installation

1. Clone the repository:

```bash
git clone https://github.com/codebyaadi/vercel-clone.git
```


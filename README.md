# Bitcoin Explorer Backend

## Summary
This is the backend component of the Bitcoin Explorer project, a RESTful Web Application that provides API endpoints to fetch Bitcoin blockchain metrics and performs continuous automatic ingestion of blockchain data into the database.

## Technologies Used
- Rust
- PostgreSQL
- Docker & Docker Compose
- GitHub Actions (CI/CD)
- AWS EC2 (Deployment)

## Prerequisites
- Docker and Docker Compose
- Git

## Running the Backend

1. Clone the repository:ning the application
git clone https://github.com/your-username/bitcoin-explorer-backend.git
cd bitcoin-explorer-backend


## Deployment
This project uses GitHub Actions for CI/CD. On every push to the main branch:
1. The application is built and tested
2. A Docker image is created and pushed to Docker Hub
3. The application is deployed to an AWS EC2 instance

The live API is accessible at: `http://vaishnavipuppala.info:8000` and `http://34.210.188.43:8000`

## Docker Configuration
- Dockerfile: Defines the container image for the backend
- docker-compose.yml: Orchestrates the backend service along with PostgreSQL and Bitcoin Core

## GitHub Actions Workflow
Located in `.github/workflows/main.yml`, it automates:
- Building and testing the Rust application
- Building and pushing the Docker image
- Deploying to AWS EC2

## Project Structure
- `src/`: Contains the Rust source code
- `main.rs`: Entry point of the application
- `api.rs`: Defines API routes
- `db.rs`: Database operations
- `bitcoin.rs`: Bitcoin RPC client operations
- `ingestion.rs`: Data ingestion logic
- `Cargo.toml`: Rust dependencies and project metadata
- `Dockerfile`: Instructions for building the Docker image
- `docker-compose.yml`: Defines services, networks, and volumes



## License
This project is open source and available under the [MIT License](LICENSE).

*This project was developed as part of the INFO7500 course*

*Author: Vaishnavi Puppala*
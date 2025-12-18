#!/bin/bash

# Create certs directory if it doesn't exist
mkdir -p certs

# Generate private key
openssl genrsa -out certs/key.pem 2048

# Generate certificate signing request
openssl req -new -key certs/key.pem -out certs/cert.csr -subj "/C=US/ST=State/L=City/O=Organization/CN=localhost"

# Generate self-signed certificate (valid for 365 days)
openssl x509 -req -days 365 -in certs/cert.csr -signkey certs/key.pem -out certs/cert.pem

# Clean up CSR
rm certs/cert.csr

echo "Self-signed certificates generated in ./certs/"
echo "cert.pem and key.pem are ready to use"

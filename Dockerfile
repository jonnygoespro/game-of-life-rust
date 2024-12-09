# Stage 1: Build
FROM node:18 AS builder

# Set the working directory inside the container
WORKDIR /app/frontend

# Copy only the required files for the build
COPY ./frontend/package.json ./frontend/package-lock.json ./

# Install dependencies
RUN npm install

# Copy the rest of the frontend directory
COPY ./frontend .

# Build the frontend application
RUN npm run build

# Stage 2: Serve
FROM node:18 AS server

# Install Vite globally for serving
RUN npm install -g vite

# Set the working directory
WORKDIR /app/frontend

# Copy the built files from the builder stage
COPY --from=builder /app/frontend/dist ./dist

# Expose the port Vite will use
EXPOSE 5173

# Serve the built files
CMD ["vite", "preview", "--port", "5173", "--host"]

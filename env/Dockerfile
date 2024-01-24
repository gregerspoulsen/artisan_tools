FROM python:3

# Install podman:
RUN apt-get update && apt-get install -y \
    podman \
    && rm -rf /var/lib/apt/lists/*


# Install Artisan Tools:
COPY . /app
WORKDIR /app
RUN pip install -e . --no-cache-dir
RUN pip install -r requirements_dev.txt
RUN pip install -r requirements_doc.txt

# Avoid issues with insonsistent file owners in mounted volumes:
RUN git config --global --add safe.directory /app
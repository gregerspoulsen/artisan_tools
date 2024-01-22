FROM python:3

COPY . /app
WORKDIR /app
RUN pip install -e . --no-cache-dir
RUN pip install -r requirements_dev.txt
RUN pip install -r requirements_doc.txt

# Avoid issues with insonsistent file owners in mounted volumes:
RUN git config --global --add safe.directory /app
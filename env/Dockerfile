FROM python:3

COPY . /app
WORKDIR /app
RUN pip install -e . --no-cache-dir
RUN pip install -r requirements_dev.txt

# Create user with id $USER_ID:
ARG USER_ID
RUN useradd -m -u $USER_ID dev

USER dev

# Add github to known hosts:
RUN mkdir -p ~/.ssh
# '~'' does not work for COPY command
COPY known_hosts /home/dev/.ssh/known_hosts
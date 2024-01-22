# --- Development Tasks ---
# List tasks
@default:
  just --list

# Run pre-commit hooks
pre-commit: 
  just test
  just format
  just lint

# Run the test suite
test target="":
  just run pytest {{target}}

# Build the development environment, relevant when dependencies change
build:
  cd env/ && docker compose build

# Run code formatter
format:
  just run black .

# Lint code
lint:
  just run flake8

# Run CI pipeline tasks
ci:
  just lint
  just test

doc:
  just run sphinx-build -b html doc/source doc/build

# --- Release Tasks ---

# Bump the version. Usage: just bump major|minor|patch
bump args: 
  just run at version bump {{args}} VERSION

# Check VERSION contains a valid semver and the tag does not exist
check-release:
  just run at version verify --check-tag

# Create a release
release: 
  just run at vcs add-release-tag

# --- Utilities ---
# Run command in development environment
run *ARGS: # Run a command in the development environment
  docker compose -f env/docker-compose.yml run --rm dev {{ARGS}}
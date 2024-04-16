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

# Run tox tests:
tox *args:
  just run tox {{args}}
  # Remove .egg-info it seems to make MANIFEST changes unreliable:
  just run "rm -rf src/artisan_tools.egg-info/"

# Build the development environment, relevant when dependencies change
env:
  cd env/ && docker compose build

# Run code formatter
format:
  just run black --preview .

# Lint code
lint:
  just run black --check .
  just run flake8
  just run mypy .
  just run pydocstyle

# Run CI pipeline tasks
ci:
  just lint
  just tox
  just doc

# Run tests locally in venv to verify docker commands
local-test *args:
  python3 -m venv .venv
  ./.venv/bin/pip install -e .
  ./.venv/bin/pip install -r requirements_dev.txt
  ./.venv/bin/pytest {{args}}

doc:
  docker compose -f env/docker-compose.yml run --rm -u $(id -u) dev sphinx-build -b html doc/source doc/build

# --- Release Tasks ---

# Bump the version. Usage: just bump major|minor|patch
bump *args:
  just run at version bump {{args}}

# Check VERSION contains a valid semver and the tag does not exist
check-release:
  just run at version verify --check-tag

# Create a release
release:
  just run at vcs add-tag

# --- Utilities ---
# Run command in development environment
run *ARGS: # Run a command in the development environment
  docker compose -f env/docker-compose.yml run --rm dev {{ARGS}}

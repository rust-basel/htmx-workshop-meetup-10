set dotenv-load := true
import 'api_tests/hurl.just'

alias v := verify
alias r := run

@default:
    just --list

run *args:
    cargo run -q -- {{args}}

# Perform all verifications (compile, test, lint, etc.)
@verify: test lint api_tests
    echo ------------ verify done! ------------    

# Run tests    
test:
    cargo test

# Run the static code analysis
lint:
    cargo fmt --all -- --check
    cargo clippy

fmt:
    cargo fmt
    
book:
    mdbook serve meetup
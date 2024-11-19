set dotenv-load := true
import 'api_tests/hurl.just'

alias v := verify
alias r := run

@default:
    just --list

watch:
    cargo watch -s 'just run'

run:
    shuttle run 

# Perform all verifications (compile, test, lint, etc.)
@verify: test lint
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
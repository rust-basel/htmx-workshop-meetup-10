

@default:
    just --list


run *args:
    cargo run -q -- {{args}}

    
# Perform all verifications (compile, test, lint, etc.)
@verify: test lint
    echo ------------ verify done! ------------    
    
# run tests    
test:
    cargo test

# Run the static code analysis
lint:
    cargo fmt --all -- --check
    cargo clippy



fmt:
    cargo fmt
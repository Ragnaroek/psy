## running psy
asm FILE OUT:
    cargo run assemble --flat -o {{OUT}} {{FILE}}

## testing psy
test:
    cargo test

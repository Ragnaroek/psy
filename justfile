# # running psy
asm FILE OUT:
    cargo run assemble --flat -o {{ OUT }} {{ FILE }}

# # testing psy
test:
    cargo test

coverage:
    cargo tarpaulin --ignore-tests --out Lcov
    genhtml lcov.info --output-directory coverage_report

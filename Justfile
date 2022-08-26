#!/usr/bin/env -S just --justfile

set dotenv-load

@default:
	# @just --list --justfile {{justfile()}} --unsorted
	echo 'Usage:'
	echo '    just <command> [options]'
	echo
	echo 'Package commands:'
	echo '    (b)uild'
	echo '    (f)mt'
	echo '    (c)heck   # run clippy'
	echo '    (t)est    # run tests with nextest'
	echo '    bench     # run criterion benchmarks'
	echo
	echo '    flame-bench <bench> <out> <filter>'
	echo '              # save a flamegraph at <out>.svg of <bench> run for 10 seconds'
	echo
	echo 'Workspace commands:'
	echo '    fmt-all (fa)'
	echo '    check-all (ca)'
	echo '    test-all (ca)'
	echo '    bench-all'
	echo
	echo '# Will always run in <project_root>/fuzzing/'
	echo 'Fuzzing:'
	echo '    fuzz <command>           # cargo hfuzz <command>'
	echo '    fuzz-run (fr) <target>   # fuzz <target>'
	echo '    fuzz-debug <target>      # debug crashes for <target>'
	echo '    defuzz                   # alias for fuzz-debug'
	echo
	echo 'Misc:'
	echo '    install-dev-deps   # install/update all necessary cargo subcommands'

alias b := build
build *args='':
	cargo build {{args}}

alias f := fmt
alias format := fmt
fmt *args='':
	cd {{invocation_directory()}} && cargo fmt {{args}}

alias fa := fmt-all
alias format-all := fmt-all
fmt-all:
	cargo fmt

alias c := clippy
alias check := clippy
clippy *args='': fmt
	cd {{invocation_directory()}} && cargo clippy {{args}}

alias ca := clippy-all
alias check-all := clippy-all
clippy-all *args='': fmt-all
	cargo clippy --workspace --all-targets {{args}}

alias t := test
test +args='': fmt
	cd {{invocation_directory()}} && cargo clippy --tests && cargo nextest run {{args}}

alias ta := test-all
test-all +args='': fmt
	cargo clippy --workspace --lib --tests && cargo nextest run {{args}}

bench *args='': fmt
	cd {{invocation_directory()}} && cargo clippy --benches && cargo criterion --benches {{args}}

bench-all *args='': fmt
	cargo clippy --workspace --lib --benches && cargo criterion --workspace --benches {{args}}

flame-bench bench out filter:
	export CARGO_PROFILE_BENCH_DEBUG=true \
		&& cd {{invocation_directory()}} \
		&& cargo clippy --benches \
		&& cargo flamegraph --deterministic --output {{out}}.svg --bench {{bench}} -- --bench --profile-time 10 '{{filter}}'

@fuzz-add target:
	echo -e "\n[[bin]]\nname = \"{{target}}\"\npath = \"fuzz_targets/{{target}}.rs\"\ntest = false\ndoc = false" >> fuzz/Cargo.toml
	mkdir -p fuzz/fuzz_targets
	echo -e "#![no_main]\n\nuse blackbox_fuzz::{encoding, fuzz_target, get_streams};\n\nfuzz_target!(|data: &[u8]| {\n    let (mut reference, mut biterator) = get_streams(bytes).unwrap();\n\n    assert_eq!(todo!(), todo!());\n});" > fuzz/fuzz_targets/{{target}}.rs
	echo 'Initialized fuzz/fuzz_targets/{{target}}.rs'

alias fls := fuzz-list
@fuzz-list:
	echo "All available fuzzing targets:"
	cargo fuzz list

fuzz-check:
	cargo +nightly fuzz check

alias frun := fuzz-run
fuzz-run target *args='':
	cargo +nightly fuzz run {{target}} {{args}}

alias fcmin := fuzz-corpus-min
fuzz-corpus-min target *args='':
	cargo +nightly fuzz cmin {{target}} fuzz/corpus/{{target}} {{args}}

alias ftmin := fuzz-test-min
fuzz-test-min target *args='':
	cargo +nightly fuzz tmin {{target}} fuzz/corpus/{{target}} {{args}}

fuzz-fmt target input *args='':
	cargo +nightly fuzz fmt {{target}} fuzz/corpus/{{target}}/{{input}} {{args}}

nightlySysroot := `rustc +nightly --print sysroot`
llvmCov := join(nightlySysroot, "lib/rustlib/*/bin/llvm-cov")

alias fcov := fuzz-coverage
fuzz-coverage target *args='':
	cargo +nightly fuzz coverage {{target}} {{args}}
	@{{llvmCov}} show \
		--format=html \
		--instr-profile=fuzz/coverage/{{target}}/coverage.profdata \
		--output-dir=fuzz/coverage/{{target}} \
		target/*/coverage/*/release/{{target}}
	@echo
	@echo "Saved coverage to fuzz/coverage/{{target}}/index.html"

install-dev-deps:
	cargo install cargo-criterion cargo-fuzz cargo-nextest flamegraph

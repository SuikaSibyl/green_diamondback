UNAME := $(shell uname)

ifeq ($(UNAME), Linux)
ARCH := elf64
endif
ifeq ($(UNAME), Darwin)
ARCH := macho64
endif

tests/%.s: tests/%.snek src/main.rs
	cargo run -- $< tests/$*.s

tests/%.run: tests/%.s runtime/start.rs
	nasm -f $(ARCH) tests/$*.s -o tests/$*.o
	ar rcs tests/lib$*.a tests/$*.o
	rustc -L tests/ -lour_code:$* runtime/start.rs -o tests/$*.run

.PHONY: test test-reference-interpreter clean
test:
	cargo build
	cargo test

target/reference-interpreter: diamondback.ml
	ocamlopt diamondback.ml -o target/reference-interpreter
	rm diamondback.{cmx,cmi,o}

test-reference-interpreter: target/reference-interpreter
	[ -z "$(git status --porcelain)" ] || ( echo 'Ensure your git working directory is clean first' ; exit 1 )
	cp reference_interpreter_shim.rs tests/infra/mod.rs
	cargo test
	git checkout tests/infra/mod.rs

clean:
	rm -f tests/*.a tests/*.s tests/*.run tests/*.o

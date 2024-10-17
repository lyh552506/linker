TEST_CASES :=$(wildcard tests/*.sh)

build:
	@cargo build
	@ln -sf ./target/debug/my_linker ./ld

test: build
	@CC="riscv64-linux-gnu-gcc" \
	$(MAKE) $(TEST_CASES)
	@printf '\e[32mPassed all tests\e[0m\n'

$(TEST_CASES):
	@echo 'Testing' $@
	@./$@
	@printf '\e[32mOK\e[0m\n'


clean:
	@cargo clean
	@rm -rf out/
	@rm ./ld

.PHONY: build clean test $(TEST_CASES)
TEST_CASES :=$(wildcard tests/*.sh)

build:
	cargo build

test: build
	$(MAKE) $(TEST_CASES)
	@printf '\e[32mPassed all tests\e[0m\n'

$(TEST_CASES):
	@echo 'Testing' $@
	@./$@
	@printf '\e[32mOK\e[0m\n'


clean:
	cargo clean
	rm -rf out/

.PHONY: build clean test $(TEST_CASES)
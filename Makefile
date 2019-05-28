#
# Option
#
LOG_LEVEL := debug
APP_ARGS := "foo%20bar"

#
# @ Env
#
export RUST_LOG=durl=$(LOG_LEVEL)

#
# Task
#
run:
	cargo run $(APP_ARGS)

test:
	cargo test

check:
	cargo check $(OPTION)
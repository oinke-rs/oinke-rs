
dev:
	cargo build
	cp target/thumbv7em-none-eabi/debug/oinke-rs target/oinke-rs.elf
o:
	Ozone $(CURDIR)/scripts/ozone.jdebug &

ko:
	kill `pidof Ozone`

fresh f:
	cargo build --features "oinke-rs_cube_rs/refresh"

check c:
	cargo check

release r:
	cargo build --release
	cp target/thumbv7em-none-eabi/release/oinke-rs target/oinke-rs.elf

prod p:
	cargo build --release --features "prod"
	cp target/thumbv7em-none-eabi/release/oinke-rs target/oinke-rs.elf

size sz:
	arm-none-eabi-size -Ax target/oinke-rs.elf
	#cargo size --bin oinke-rs --release

objdump:
	cargo objdump --bin oinke-rs --release --features prod -- -S | less

clean:
	rm -rf target/


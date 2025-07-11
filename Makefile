start_first:
	cargo run -- 127.0.0.1:8080 127.0.0.1:8081 r1zzd

start_second:
	cargo run -- 127.0.0.1:8081 127.0.0.1:8080 r1zzd2

start_third:
	cargo run -- 127.0.0.1:8082 127.0.0.1:8081 r1zzd3
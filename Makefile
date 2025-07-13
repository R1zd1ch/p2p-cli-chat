start_vpn_first:
	cargo run -- 10.8.1.2:8080 10.8.1.2:8081 r1zzd2  my_secret_token

start_vpn_second:
	cargo run -- 10.8.1.2:8081 10.8.1.2:8080 r1zzd_bebra my_secret_token
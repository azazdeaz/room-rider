web-proxy:
	grpcwebproxy \
	--run_tls_server=false \
	--backend_addr=localhost:10000 \
	--backend_max_call_recv_msg_size=50242880 \
	--allow_all_origins \
	--server_http_max_write_timeout=1h
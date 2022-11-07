# dynv6-docker

## docker-compose file example
```
version: "3"

services:
  dynv6-docker:
    image: kotahv/dynv6-docker:rust
    container_name: dynv6-docker
    network_mode: host
    environment:
      - dynv6_hostname=<your domain>
      - dynv6_token=<your dynv6 token>
      - dynv6_interval=600
      - dynv6_no_ipv4=true
      - dynv6_interface=eth0
      - dynv6_log_level=INFO
    volumes:
      - <Your Path>:/opt/dynv6/data
    restart: unless-stopped
```
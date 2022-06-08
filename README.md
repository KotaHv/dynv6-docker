# dynv6-docker

## docker-compose file example
```
version: "3"

services:
  dynv6-docker:
    image: kotahv/dynv6-docker:latest
    container_name: dynv6-docker
    network_mode: host
    environment:
      - hostname=<your domain>
      - dynv6_token=<your dynv6 token>
      - interval=600
      - no_ipv4=true
      - interface=eth0
    restart: unless-stopped
```
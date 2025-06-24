set shell := ["powershell.exe", "-c"]

setup:
    -docker network create appnet

run:
  just setup
  docker-compose -f server/compose.yaml up -d
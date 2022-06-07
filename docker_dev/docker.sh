#!/bin/sh
docker build -t huest . 
docker run -it -p 80:8081 -p 443:443 -p 1900:1900 -e ROCKET_PROFILE=release huest bash
# docker-compose run -p 80:80 -p 443:443 -p 1900:1900 huest git clone https://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@github.com/gmallios/huest.git&&/bin/sh

FROM alpine:3.16 AS base

RUN apk add --no-cache --update python3

FROM python:3.10 AS install

COPY requirements.txt .

RUN pip install -r requirements.txt

FROM base

COPY --from=install /usr/local/lib/python3.10/site-packages /usr/lib/python3.10/site-packages

WORKDIR /opt/dynv6

COPY main.py .

ENTRYPOINT python3 main.py

FROM alpine:3.16 AS base

RUN apk add --no-cache --update python3 tzdata

FROM python:3.10 AS install

COPY requirements.txt .

RUN pip install -r requirements.txt --no-binary pydantic

RUN rm -rf /usr/local/lib/python3.10/site-packages/pip*

RUN rm -rf /usr/local/lib/python3.10/site-packages/setuptools*

FROM base

COPY --from=install /usr/local/lib/python3.10/site-packages /usr/lib/python3.10/site-packages

WORKDIR /opt/dynv6

COPY src/ .

ENTRYPOINT ["python3", "main.py"]

FROM python:alpine

RUN adduser -D -u 1000 app

USER app
WORKDIR /app

RUN python -m venv /app/.venv
ENV PATH="/app/.venv/bin:$PATH"

RUN pip install --no-cache-dir --upgrade pip \
  && pip install --no-cache-dir sphinx

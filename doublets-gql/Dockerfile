FROM golang:1.21 AS builder

WORKDIR /app
RUN git clone https://github.com/linksplatform/Data.Doublets.Gql.git .
RUN go build -o server ./cmd/doublets-gql

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates

COPY --from=builder /app/server /usr/local/bin/doublets-gql

EXPOSE 9090

CMD ["doublets-gql"]
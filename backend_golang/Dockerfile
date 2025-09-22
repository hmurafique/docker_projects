FROM golang:1.20 AS builder
WORKDIR /app
COPY . .
RUN go mod init backend_golang && go mod tidy
RUN go build -o server .

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/server .
EXPOSE 5000
CMD ["./server"]

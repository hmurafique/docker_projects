FROM golang:1.20 AS builder
WORKDIR /app
COPY . .
RUN go mod init backend_golang || true
RUN go mod tidy
# Static build
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -o server .

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/server .
EXPOSE 5000
CMD ["./server"]

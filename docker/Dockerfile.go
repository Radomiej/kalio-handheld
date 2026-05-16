# Go dev-server container
FROM golang:1.22-alpine

RUN apk add --no-cache git

WORKDIR /workspace/tools/dev-server
COPY tools/dev-server/go.mod ./
RUN go mod download 2>/dev/null || true

COPY tools/dev-server/ ./

CMD ["go", "run", ".", "/workspace/apps", "8765"]

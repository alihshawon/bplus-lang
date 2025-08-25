.PHONY: build test clean

VERSION ?= dev
COMMIT  := $(shell git rev-parse --short HEAD 2>/dev/null || echo 'none')
DATE    := $(shell date -u +'%Y-%m-%dT%H:%M:%SZ')

LDFLAGS := -X github.com.alihshawon.bplus-lang/cli/internal/version.Version=$(VERSION)                    -X github.com.alihshawon.bplus-lang/cli/internal/version.Commit=$(COMMIT)                    -X github.com.alihshawon.bplus-lang/cli/internal/version.Date=$(DATE)

build:
	go build -ldflags "$(LDFLAGS)" -o bin/bplus ./cmd/bplus
	go build -ldflags "$(LDFLAGS)" -o bin/bpm ./cmd/bpm

test:
	go test ./...

clean:
	rm -rf bin

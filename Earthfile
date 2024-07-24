VERSION 0.7


COPY_CI_DATA:
    COMMAND
    COPY --dir "ci/" ".github/" "./"


golang-base:
    FROM golang:1.22.1
    ENV GOPROXY=direct
    ENV CGO_ENABLED=0
    ENV GOOS=linux
    ENV GOARCH=amd64


check-github-actions-workflows-linting:
    FROM +golang-base
    RUN go install github.com/rhysd/actionlint/cmd/actionlint@v1.6.26
    DO +COPY_CI_DATA
    RUN ./ci/check-github-actions-workflows-linting.sh


check-linting:
    BUILD +check-github-actions-workflows-linting

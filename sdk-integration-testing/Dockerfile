FROM golang:1.16 as mock-build

WORKDIR /build

ADD . .

RUN go build -o mock-api -i mock-api.go

RUN go build -o verify -i sdk-compatibility-testing/verify-exemplar.go

FROM registry.cto.ai/official_images/bash:latest

COPY --from=mock-build /build/mock-api /bin/mock-api
COPY --from=mock-build /build/verify /bin/verify

WORKDIR /ops

ADD sdk-compatibility-testing/bash .

CMD /bin/verify bash ./exemplar.sh

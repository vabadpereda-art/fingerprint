cargo run --bin test_scanner



docker build -f Dockerfile.debian12 -t zkfp-debian12 .


mkdir -p dist/debian12 && docker create --name zkfp_extract zkfp-debian12 && docker cp zkfp_extract:/out/. dist/debian12/ && docker rm zkfp_extract && cp dist/debian12/libzkfp_capi.so java-client/sdk/libzkfp_capi.so && ls -lh dist/debian12/ java-client/sdk/libzkfp_capi.so
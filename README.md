# get-photos-lambda

AWS Lambda to retrieves images from Photos DB.

It is intended for this to integration an HTTP API with DynamoDB.

## Test

Currently, there are no tests for this code. Pure cowboy stuff at the moment!

```commandline
cargo test
```

## Build

To build the Rust binary

```commandline
$ cargo build --release
```

### Build on MacOS

```commandline
$ cargo build --release --target x86_64-unknown-linux-musl
```

To build the AWS Lambda zip file

```commandline
$ cp ./target/x86_64-unknown-linux-musl/release/get-photos-lambda ./bootstrap && zip lambda.zip bootstrap && rm bootstrap
```

## Deployment

Create AWS Lambda for the first time

```commandline
$ aws lambda create-function --function-name get-photos \
  --handler not.used.in.rust \
  --zip-file fileb://./lambda.zip \
  --runtime provided \
  --role arn:aws:iam::409276471434:role/get-photos-lambda \
  --environment Variables={RUST_BACKTRACE=1} \
  --tracing-config Mode=Active
```

To update the code of an existing AWS Lambda

```commandline
$ aws lambda update-function-code --function-name get-photos \
--zip-file fileb://./lambda.zip
```

## Invoke AWS Lambda

To test your deployed AWS Lambda works correctly

```commandline
$ aws lambda invoke --function-name get-photos \
  --cli-binary-format raw-in-base64-out \
  --payload '{"firstName": "world"}' \
  output.json
$ cat output.json  # Prints: {"message":"Hello, world!"}
```

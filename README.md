# File Reader Processor

## Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Usage](#usage)
- [Contributing](../CONTRIBUTING.md)

## About <a name = "about"></a>

A Dapr MicroService that provides CRUD functionality for ReconTasks.

## Getting Started <a name = "getting_started"></a>

Clone the repo

### Prerequisites

```
- Dapr
- Rust
```

### Installing

A step by step guide to get a development env running.

Run dapr

```
daprd --app-id svc-task-details-repository-manager  --app-port 8080 --dapr-http-port 3500 --components-path "./dapr-components" --dapr-grpc-port 5005
```

Build the app

```
cargo build
```

Run Tests

```
cargo test
```

Run the app

```
cargo run
```

Sample Read Primary File Request

```
curl --location --request POST 'http://localhost:8082/read-file' \
--header 'Content-Type: application/json' \
--data-raw '{
    "file": {
        "file_storage_location": "LocalFileSystem",
        "file_extension": "Csv",
        "file_path": "E:\\Work\\cplk\\primary_file.csv",
        "file_type": "PrimaryFile",
        "file_metadata": {
            "comparison_pairs": [
                {
                    "primary_file_column_index": 0,
                    "comparison_file_column_index": 0,
                    "is_row_identifier": true
                }
            ]
        }
    }
}'
```

Sample Read Comparison File Request

```
curl --location --request POST 'http://localhost:8082/read-file' \
--header 'Content-Type: application/json' \
--data-raw '{
    "file": {
        "file_storage_location": "LocalFileSystem",
        "file_extension": "Csv",
        "file_path": "E:\\Work\\cplk\\primary_file.csv",
        "file_type": "ComparisonFile",
        "upload_request_id": "RECON-TASK-10f5c31a-515e-42f9-8151-86eba2cacce8"
    }
}'
```

## Usage <a name = "usage"></a>

Add notes about how to use the system.

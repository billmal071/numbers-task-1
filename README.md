# Stage 1 Number Classification API

## Description
This is a simple API that classifies numbers. The API is built using Actix Web and is deployed on Render. The API is available at https://stage1-number-classification.herokuapp.com/.

## Usage
The API has a single endpoint `/classify-number` that accepts a POST request with a JSON body containing a single number. The API classifies the number as either `even` or `odd` and returns the result in a JSON response.

### Request
```json
{
  "number": 5
}
```

### Response
```json
{
  "number": 5,
  "classification": "odd"
}
```

## Development
To run the API locally, you need to have Rust installed. You can install Rust using [rustup](https://rustup.rs/). After installing Rust, you can run the following commands to start the API:

```bash
git clone 

[![Build Status](https://travis-ci.org/AlesTsurko/image_uploader.svg?branch=master)](https://travis-ci.org/AlesTsurko/image_uploader)

# image_uploader
A demo of image uploader server written in Rust with [actix-web](https://github.com/actix/actix-web).

Supports:
- JPEG
- BMP
- GIF
- PNG 

The next methods are allowed:
- send binary data directly (single file upload only);
- using multipart-form/data;
- using JSON request with base64 encoded string;
- from destination URL.


## Usage

```
$ image_uploader --help
Image Uploader 0.1.0
Ales Tsurko
Image uploader server demo.

USAGE:
    image_uploader [OPTIONS] --bind_to <ADDRESS>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --bind_to <ADDRESS>      127.0.0.1:8000 for ex.
    -s, --storage_path <PATH>    Specify path where data will be stored. If not specified default path (./storage) will
                                 be used.
```

## API

You should properly set the "Content-Type" field of your request's header. The possible values are:

- `image/format` for direct uploads;
- `application/json` for json requests;
- `multipart/form-data` for form-data.

### `PUT /upload`

#### Response

| Name       | Type            | Example | Description |
| ----       | ----            | ------- | :---------- |
| `ids`      | `Array<String>` | `[]`    |             |
| `previews` | `Array<String>` | `[]`    |             |

### `GET /:id`

Responses with image.

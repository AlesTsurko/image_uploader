[![Build Status](https://travis-ci.org/AlesTsurko/image_uploader.svg?branch=master)](https://travis-ci.org/AlesTsurko/image_uploader)

# image_uploader
A demo of an image uploader server written in Rust with [actix-web](https://github.com/actix/actix-web).

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

Using docker (will run on port `8000`):
```
$ docker-compose up
```

Or install and:
```
$ image_uploader --help
Image Uploader 0.1.0
Ales Tsurko
An image uploader server demo.

USAGE:
    image_uploader [OPTIONS] --bind_to <ADDRESS>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --bind_to <ADDRESS>      127.0.0.1:8000 for ex.
    -s, --storage_path <PATH>    Specifies the path where to store the data. If not specified the default one
                                 (./storage) will be used.
```

## API

You should properly set the "Content-Type" field of your request's header. The possible values are:

- `image/format` for direct uploads;
- `application/json` for json requests;
- `multipart/form-data` for form-data.

For base64 strategy JSON request body model is:

| Name   | Type            | Example |
| ----   | ----            | ------- |
| `data` | `Array<String>` |         |

### `PUT /upload`

#### Response

| Name  | Type            | Example                                    |
| ----  | ----            | -------                                    |
| `ids` | `Array<String>` | `["936da01f-9abd-4d9d-80c7-02af85c822a8"]` |

### `PUT /upload?url=:url`

Uploads an image from the given URL.

#### Response

The same as for `PUT /upload`

### `GET /:id`

Responses with an image.

### `GET /:id?preview`

Responses with an image preview.

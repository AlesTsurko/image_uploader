# image_uploader
A demo of image uploader REST API written in Rust with [actix-web](https://github.com/actix/actix-web).

Three methods are allowed:
- send binary data directly;
- using multipart-form/data;
- using JSON request with base64 encoded string.

> The "Content-Type" field of your request's header have to be set when you send data directly.

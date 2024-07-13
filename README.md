
# try-rust.org


## Running your project

```bash
npx tailwindcss -i input.css -o ./style/output.css --watch
```

```bash
cargo leptos watch
```

## Compiling for Release
```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

## Docker

```bash
docker build . -t tryrust
```

```bash
docker run -p8080:8080 tryrust 
```


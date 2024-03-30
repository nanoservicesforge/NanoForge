# NanoForge
Build and runner tool for nanoservices


## Installation

We want to work on packaging for operating systems later but for now you can install using the following command:

```bash
wget -qO- https://raw.githubusercontent.com/nanoservicesforge/NanoForge/main/scripts/install.sh | sh
```

## Usage

Before we can use NanoForge we need to package a nanoservice and build it using `scratch` as seen below:

```Dockerfile
FROM scratch

COPY ./your_package .
```

## Declaring a nanoservice in your build

And this is enough to package your nanoservice in a Docker image. Now we move onto declaring our nanoservice in our
`Cargo.toml` with the following (`nan-one` is a real toy example of a nanoservice on Docker Hub):

```toml
[nanoservices.nan-one]
dev_image = "maxwellflitton/nan-one"
prod_image = "maxwellflitton/nan-one"
entrypoint = "."
```

## Preparing your build

Now we can prepare our build using the following command (you pwd should be the root of your project):

```bash
nanoforge prep
```

This command will create a `.nanoservices_cache` and pull the Docker image unpacking the files into the cache. The
build tool will then scan all subdirectories looking for nanoservices in all the `Cargo.toml` files in the project.
The relative path will then be calculated for each nanoservice and defined in the `Cargo.toml` files. For instance,
our `nan-one` nanoservice will have the following path:

```toml
[dependencies.nan-one]
path = "../.nanoservices_cache/domain_services/nanoservices/maxwellflitton_nan-one/."
```

You can then use this in your build.

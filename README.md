# Node.js vs. Rust: A Thumbnail Service Benchmark

I built this project to solve a problem with my self-hosted photo backup system, which runs on a Raspberry Pi. The original Node.js thumbnail service was slow and failed completely on photos from newer iPhones.

This repo contains a simple, containerized benchmark I created to compare the original Node.js implementation against a new one written in Rust.

For the full story of the investigation, the tricky dependency bugs I found, and a deep dive into the results, check out the full blog post here [insert blog link].

## How To Use

First, add some images to the `/images` folder.

### Run the Node.js Benchmark

```
docker compose up --build node-app
```

### Run the Rust Benchmark

```
docker compose up --build rust-app
```

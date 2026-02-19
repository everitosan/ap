#!/bin/bash

build_name="ap.7z";
build_dir="builds";

mkdir -p "${build_dir}";

# --------------
# Build jobs
# --------------

# Build front
cd front;
npm run build;

# Build back
cd ../back;
cargo build --release;
cd ../;

# --------------
# Build package
# --------------

ls -lah front/apps/website/dist;

cd front/apps/website/dist && 7z a "${build_name}" ./;

mv "${build_name}" ../../../../back/target/release;
cd ../../../../back/target/release;

7z a "${build_name}" ap-back;
mv "${build_name}" "../../../${build_dir}";
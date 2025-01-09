#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Scan the packages directory and list available packages
echo "Available packages:"
packages=()
index=1
for dir in packages/*; do
  if [ -d "$dir" ]; then
    package_name=$(basename "$dir")
    packages+=("$package_name")
    echo "$index) $package_name"
    ((index++))
  fi
done

# Prompt the user to select a package
read -p "Enter the number of the package you want to publish: " package_index

# Validate the input
if ! [[ "$package_index" =~ ^[0-9]+$ ]] || [ "$package_index" -lt 1 ] || [ "$package_index" -gt "${#packages[@]}" ]; then
  echo "Invalid selection. Exiting."
  exit 1
fi

# Publish the selected package
selected_package="${packages[$((package_index - 1))]}"
echo "Publishing package: $selected_package"
cd "packages/$selected_package"
cargo publish

echo "Package $selected_package has been published."
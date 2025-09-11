#!/usr/bin/env bash
set -euo pipefail

# --- Extract repo info ---
COMMIT_HASH=$(git rev-parse HEAD)
USERNAME=$(git config user.name | tr '[:upper:]' '[:lower:]' | tr -d ' ')
REPO_NAME=$(basename -s .git "$(git config --get remote.origin.url)")
IMAGE_NAMESPACE="ghcr.io/$USERNAME/$REPO_NAME"

# --- Targets to build ---
TARGETS=("backend" "proxy" "cache")

# --- Ensure buildx is available ---
if ! docker buildx version &>/dev/null; then
  echo "docker buildx not available. Install Docker Buildx first."
  exit 1
fi

# --- Build missing images ---
for target in "${TARGETS[@]}"; do
  IMAGE_TAG="$IMAGE_NAMESPACE/$target:$COMMIT_HASH"

  if docker image inspect "$IMAGE_TAG" >/dev/null 2>&1; then
    echo "Image already exists: $IMAGE_TAG"
  else
    echo "Building $target..."
    docker buildx build \
      --target "$target" \
      -t "$IMAGE_TAG" \
      .
  fi
done

# --- Ask if pushing ---
read -rp "Do you want to push the images to GHCR? [y/N] " answer
if [[ "$answer" =~ ^[Yy]$ ]]; then
  if [[ -z "${GHCR_PAT:-}" ]]; then
    echo "GHCR_PAT not set. Cannot push."
    exit 1
  fi

  echo "$GHCR_PAT" | docker login ghcr.io -u "$USERNAME" --password-stdin

  for target in "${TARGETS[@]}"; do
    IMAGE_TAG="$IMAGE_NAMESPACE/$target:$COMMIT_HASH"
    echo "Pushing $IMAGE_TAG..."
    docker push "$IMAGE_TAG"
  done
else
  echo "Skipping push."
fi

# --- Ask if tagging commit ---
read -rp "Do you want to create a git tag prebuilt/$COMMIT_HASH? [y/N] " tag_answer
if [[ "$tag_answer" =~ ^[Yy]$ ]]; then
  git tag "prebuilt/$COMMIT_HASH"
  echo "Created git tag prebuilt/$COMMIT_HASH"
fi

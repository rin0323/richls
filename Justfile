git_revision := `git rev-parse --short HEAD`
app_version := `awk -F'"' '/^\[package\]/{p=1} p && /^version *=/{print $2; exit}' Cargo.toml`
app_license := `awk -F'"' '/^\[package\]/{p=1} p && /^license *=/{print $2; exit}' Cargo.toml`
build_date := `date -u +%Y-%m-%dT%H:%M:%SZ`

container_runner := "docker"
container_image := "ghcr.io/rin0323/richls"

# レシピ一覧を表示する
default:
    @just --list

# テストとカバレッジ測定
test:
    cargo llvm-cov

# 通常のリリースビルド
build: test
    cargo build --release

# ローカル確認用のDockerイメージを作成する
container-local:
    {{container_runner}} build \
        --build-arg GIT_REVISION="{{git_revision}}" \
        --build-arg BUILD_DATE="{{build_date}}" \
        --build-arg VERSION="{{app_version}}" \
        --build-arg LICENSE="{{app_license}}" \
        -t "{{container_image}}:latest" \
        -t "{{container_image}}:{{app_version}}" \
        -f Containerfile \
        .

# amd64・arm64用イメージを作成してGHCRへ公開する
container:
    {{container_runner}} buildx build --push \
        --platform linux/amd64,linux/arm64 \
        --build-arg GIT_REVISION="{{git_revision}}" \
        --build-arg BUILD_DATE="{{build_date}}" \
        --build-arg VERSION="{{app_version}}" \
        --build-arg LICENSE="{{app_license}}" \
        -t "{{container_image}}:latest" \
        -t "{{container_image}}:{{app_version}}" \
        -f Containerfile \
        .

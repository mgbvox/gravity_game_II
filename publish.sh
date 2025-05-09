export APP_BUCKET="gravitygame"
aws s3 mb s3://$APP_BUCKET           # create

# sync everything
aws s3 sync dist/ s3://$APP_BUCKET/ --exclude '*.wasm' --cache-control 'public,max-age=31536000,immutable'   # hashed files

# upload the wasm with its MIME type (and mark it long‑cache)
aws s3 cp dist/gravity_game-*.wasm s3://$APP_BUCKET/ --content-type application/wasm --cache-control 'public,max-age=31536000,immutable'

# keep ttl on index.html short so things go live quick
aws s3 cp dist/index.html s3://$APP_BUCKET/ --cache-control 'public,max-age=60'

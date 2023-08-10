set -e

COMMIT_TAG=$1

if [ -z "$COMMIT_TAG" ]; then 
  echo "No commit tag provided." 
  exit 0
fi

VERSION=$(npx semver $COMMIT_TAG)
if [ -z "$VERSION" ]; then 
  echo "$COMMIT_TAG is not a semantic version." 
  exit 1
fi

echo "Updating Aztec dependencies to version $VERSION"

TMP=$(mktemp)
for PKG in $(jq --raw-output ".dependencies | keys[] | select(contains(\"@aztec/\"))" package.json); do
  jq --arg v $VERSION ".dependencies[\"$PKG\"] = \$v" package.json > $TMP && mv $TMP package.json
done
#!/usr/bin/env bash
# Builds the macOS app, signs it with a Developer ID, notarizes it and uploads it to the GitHub
# release of the checked-out tag. It runs on a Mac instead of CI so the certificate and the
# notary credentials never leave the keychain.
set -euo pipefail

: "${APPLE_SIGNING_IDENTITY:?set it to a Developer ID Application identity (security find-identity -v -p codesigning)}"
: "${NOTARY_PROFILE:?set it to a notarytool keychain profile (xcrun notarytool store-credentials)}"

root="$(git rev-parse --show-toplevel)"
version="$(node -p 'require(process.argv[1]).version' "$root/app/package.json")"
tag="v$version"

if [[ "$(git -C "$root" describe --exact-match --tags HEAD 2>/dev/null)" != "$tag" || -n "$(git -C "$root" status --porcelain)" ]]; then
  echo "Check out $tag with a clean tree first, so the build matches the tagged source." >&2
  exit 1
fi

cd "$root/app"
npm ci
npx tauri build --target universal-apple-darwin --bundles dmg

dmg="$root/app/src-tauri/target/universal-apple-darwin/release/bundle/dmg/Home of Builders_${version}_universal.dmg"
xcrun notarytool submit "$dmg" --keychain-profile "$NOTARY_PROFILE" --wait
xcrun stapler staple "$dmg"
spctl --assess --type open --context context:primary-signature --verbose "$dmg"

gh release view "$tag" >/dev/null 2>&1 ||
  gh release create "$tag" --draft --verify-tag --title "Home of Builders $tag" --notes "See CHANGELOG.md for what changed."
gh release upload "$tag" "$dmg" --clobber
echo "Uploaded to release $tag. Review it on GitHub and publish."

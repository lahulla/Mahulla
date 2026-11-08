
#!/usr/bin/env bash
set -euo pipefail

MSG="${1:-message}"

git add .
git commit -m "$MSG"
git branch -M main
git remote add origin git@github.com:lahulla/Mahulla.git 2>/dev/null || git remote set-url origin git@github.com:lahulla/Mahulla.git
git push -u origin main

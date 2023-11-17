#!/bin/zsh
source ~/.zshrc
nvm use --delete-prefix v18.17.1
echo "running aztec-cli dap"
node ~/work/aztec-packages/yarn-project/cli/dest/bin/index.js dap
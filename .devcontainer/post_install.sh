# Make sure rust is up to date
rustup update
# Install rustling 
# I had issues during fetch under osx, the below config seems to fix it
cargo -v --config net.git-fetch-with-cli=true install --force --path /workspaces/rustlings
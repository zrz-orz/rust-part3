cd cluster

cargo run set zju 3 2>/dev/null
echo "set zju 3"
cargo run set sjtu 3 2>/dev/null
echo "set sjtu 3"
cargo run set fdu 3 2>/dev/null
echo "set fdu 3"
cargo run set pku 1 2>/dev/null
echo "set pku 1"

../cluster/target/debug/cluster get pku
echo "get pku"

../cluster/target/debug/cluster get zju
echo "get zju"

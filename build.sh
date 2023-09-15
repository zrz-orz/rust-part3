#!/bin/bash

cd myredis
cargo build

cd ..
cd proxy
cargo build
cd ..

./myredis/target/debug/server 7000 &
./myredis/target/debug/server 7001 &
./myredis/target/debug/server 7002 &
./myredis/target/debug/server 7003 &
./myredis/target/debug/server 7004 &
./myredis/target/debug/server 7005 &
./myredis/target/debug/server 7006 &
./myredis/target/debug/server 7007 &
./myredis/target/debug/server 7008 &
./myredis/target/debug/server 7009 &

alias redis='./cluster/target/debug/cluster'

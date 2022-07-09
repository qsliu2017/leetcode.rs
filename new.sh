#!/bin/sh
set -e
lib=src/lib.rs
new(){
    local mod=$(printf "_%04d" "$1")
    
    cp -n template src/$mod.rs || echo "src/$mod.rs already exists" && exit 1
    
    echo "mod $mod;" >> $lib
    sort $lib -o $lib.tmp
    mv $lib.tmp $lib
    
    echo create $1
}

new $1

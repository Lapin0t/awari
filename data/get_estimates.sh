#!/bin/bash

function do_things {
    tmp=tmp-${1}-${2}
    CARGO_TARGET_DIR=$tmp AWARI_SEEDS=$2 AWARI_PITS=$1 cargo build -j 1 --bin stats --release &> /dev/null
    CARGO_TARGET_DIR=$tmp AWARI_SEEDS=$2 AWARI_PITS=$1 cargo build -j 1 --bin awari --release &> /dev/null

    estimate=$($tmp/release/stats 1000000 | tail -n1 | awk ' { print $NF } ')
    bool=$(echo "$estimate < 500000000" | bc -l)
    if [[ $bool -eq 1 ]] ; then
        real=$($tmp/release/awari $2 --quiet | tail -n1 | awk ' { print $NF } ')
    else
        real=""
    fi
    printf '%2d %2d %15.3f %15d\n' $1 $2 $estimate $real
}

export -f do_things
parallel do_things ::: $(seq 2 6) ::: $(seq 8 4 48)

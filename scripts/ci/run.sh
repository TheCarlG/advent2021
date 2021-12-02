#!/bin/bash
for e in $(find target/release/ -maxdepth 1 -executable -type f -name 'day*' | sort); do
        n=$(echo $e | cut -d/ -f3)
        i=0
        echo "::set-output name=$n-name::**$n**"
        $e | while IFS= read -r l; do 
                echo "::set-output name=$n-result-$i::  $(echo $l | tr '\n' ' ')"
                ((i++))
        done
        ((i++))
        echo "::set-output name=$n-result-$i::-"
done

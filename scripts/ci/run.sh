#!/bin/bash
i=0
for e in $(find target/release/ -maxdepth 1 -executable -type f -name 'day*' | sort); do
  echo "::set-output name=name$i::**$(echo $e | cut -d/ -f3)**"
  echo "::set-output name=result$i::$($e | tr '\n' "|")"
  ((i++))
done

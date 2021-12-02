#!/bin/bash
for e in $(find target/release/ -maxdepth 1 -executable -type f -name 'day*' | sort); do
        n=$(echo $e | cut -d/ -f3)
        echo "::set-output name=$n-name::**$n**"
        i=0
        echo "::set-output name=$n-result-$i::\`\`\`"
        $e | while IFS= read -r l; do 
                ((i++))
                echo "::set-output name=$n-result-$i::$(echo $l | tr '\n' ' ')"
        done
        echo "::set-output name=$n-result-4::\`\`\`"
done

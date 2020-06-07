#!/bin/bash

providers=(
    'Empty Trash'
)
descriptions=(
    'Empty your trash bin instantly'
)

function buildrs {
    echo "fn main() {\
        println!(\"cargo:rustc-env=PROVIDER_NAME=$1\");
        println!(\"cargo:rustc-env=PROVIDER_DESC=$2\");
        println!(\"cargo:rustc-env=SCRIPT_PATH=$3\");
    }" > build.rs
}

function clean_buildrs {
    rm build.rs
}

mkdir -p outcome
for index in ${!providers[@]}; do
    provider=${providers[$index]}
    provider_name=`echo "$provider" | tr '[:upper:]' '[:lower:]' | tr " " "_"`
    desc=${descriptions[$index]}
    path="`pwd`/scripts/$provider_name"
    buildrs "$provider" "$desc" "$path" 
    PROVIDER_NAME=$provider PROVIDER_DESC=$desc SCRIPT_PATH=$path\
        cargo build --release
    icon=$"resources/$provider_name.png"
    mkdir -p temporary
    mkdir -p temporary/executables
    if [ -f $icon ]; then
        mkdir -p temporary/resources
    fi
    cp target/release/applescript_wrapper temporary/executables/main
    cp "profiles/$provider_name.yaml" temporary/profile.yaml
    if [ -f $icon ]; then
        cp $icon temporary/resources/icon.png
    fi
    cd temporary
    zip -r "$provider_name.zip" .
    mv "$provider_name.zip" ../outcome
    cd ..
    rm -r temporary
done
clean_buildrs

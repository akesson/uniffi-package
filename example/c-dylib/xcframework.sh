#!/bin/bash

set -e

rm -rf build
mkdir build

clang -dynamiclib libmymath.c -o build/libmymath.1.dylib -current_version 1.0 -compatibility_version 1.0

clang app.c -o build/app build/libmymath.1.dylib

echo
echo "########## deps [otool -L app]"
echo

otool -L build/app


echo
echo "########## symbols [nm libmymath.1.dylib]:"
echo

nm build/libmymath.1.dylib

echo
echo "########## output [./app]:"
echo

./build/app

xcodebuild -create-xcframework \
-library build/libmymath.1.dylib \
-headers include/ \
-output ./build/libmymath.xcframework
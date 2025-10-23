#!/bin/sh

# Output of bindgen for Objective-C depends on objc package so we need to exclude objc-dependent parts by hand.

bindgen \
  --output ./src/metal.rs \
  --with-derive-default \
  --no-derive-debug \
  --blocklist-file ./include/IUnityInterface.h \
  ./include/IUnityGraphicsMetal.h -- -x objective-c

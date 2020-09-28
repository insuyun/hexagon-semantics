#!/bin/bash

if [ ! -d qemu-hexagon ]; then
  git clone https://github.com/revng/qemu-hexagon.git

  pushd qemu-hexagon
  patch -p0 < ../qemu-hexagon.patch
  popd
fi

cd qemu-hexagon
make docker-test-quick@debian-hexagon-cross DEBUG=1

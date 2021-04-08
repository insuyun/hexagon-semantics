FROM qemu/debian-hexagon-cross

COPY qemu.tar /tmp
RUN mkdir /tmp/qemu && tar -C /tmp/qemu -xf /tmp/qemu.tar

RUN mkdir /tmp/qemu/build
WORKDIR /tmp/qemu/build
RUN ../configure --enable-werror --target-list=hexagon-linux-user
RUN make -j4
RUN make install
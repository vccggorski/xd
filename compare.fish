#!/usr/bin/env fish

set DECOMPRESSOR target/release/xd
set INPUT /usr/bin/docker
set OUTPUT /tmp/docker

lzma -c $INPUT | $DECOMPRESSOR > $OUTPUT && \
cat \
    (du -b $INPUT $OUTPUT | psub) \
    (diff (hexdump -C $INPUT | psub) (hexdump -C $OUTPUT | psub) | psub) \
    | \
bat -l diff

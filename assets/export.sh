#!/usr/bin/env nix-shell
#!nix-shell -i bash -p bash -p scour

scour ./title.inkscape.svg ./title.svg \
    --create-groups \
    --strip-xml-prolog \
    --remove-titles \
    --remove-descriptions \
    --remove-descriptive-elements \
    --enable-comment-stripping \
    --enable-viewboxing \
    --indent=none \
    --no-line-breaks \
    --strip-xml-space \
    --enable-id-stripping \
    --shorten-ids

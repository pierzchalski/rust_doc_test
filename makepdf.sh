#! /usr/bin/bash

TEST="$(pwd)/doc/lib/.*html"

echo "Test: $TEST"

ARGS=$(for html in $(find . -name $TEST); \
  do echo "page $html --javascript-delay 1000" ; \
  done)

echo "Args: $ARGS"

# wkhtmltopdf $ARGS out.pdf

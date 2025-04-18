#!/bin/bash

(
cat << EOF
(define module "test")
(define test (lambda (f a b)
  (if
    (= a b)
    (write-line "[32mPass[0m: " module ":" f)
    (begin
      (write-line "[31mFail[0m: " module ":" f)
      (write-line "  Expected: " b)
      (write-line "  Actual: " a)
      (exit)
    ))))
EOF

for filename in src/*.rs; do
  module=$(basename $filename .rs)
  echo "(define module \"$module\")"
  cat $filename | grep "^\/\/-"
done

for filename in test/*.lich; do
  module=$(basename $filename .lich)
  echo "(define module \"$module\")"
  cat $filename
done

cat << EOF
(write-line "All tests passed")
EOF
) | sed 's/^\/\/- //g' > out && cargo run -- out

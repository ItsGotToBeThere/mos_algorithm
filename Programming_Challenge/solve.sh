for i in {21..50}; do echo Solving $i; cat ./tests/test$i.in | cargo run -r 2>/dev/null > tests/test$i.out;  done

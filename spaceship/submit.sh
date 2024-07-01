for i in `seq 1 25`; do
    n=$(printf "%02d" $i)
    cargo run -r < ../problems/spaceship/$n.txt > test.txt
    python submit.py $n test.txt
done

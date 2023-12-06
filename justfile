lint day:
    cargo fmt
    cargo clippy -p day-{{day}}


create day:
    cargo generate --path ./daily-template --name day-{{day}}
    aoc download -d {{day}} --overwrite --input-file ./day-{{day}}/input1.txt --puzzle-file ./day-{{day}}/puzzle.md
    cp ./day-{{day}}/input1.txt ./day-{{day}}/input2.txt

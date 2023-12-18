#!/bin/bash

if [ -z "$1" ]; then
    CURRENT_DAY=$(TZ='America/New_York' date +%d)
else
    CURRENT_DAY=$(printf "%02d" "$1")
fi

TEMPLATE_PATH="answers/template.rs"
ANSWER_PATH="src/answer.rs"
CURRENT_YEAR=$(TZ='America/New_York' date +%Y)
CURRENT_DAY_NAME=day${CURRENT_DAY}
CURRENT_DAY_ANS_PATH="answers/${CURRENT_YEAR}/day${CURRENT_DAY}.rs"
INPUT_FILE_PATH=inputs/${CURRENT_YEAR}/day${CURRENT_DAY}.txt

# if [ ! -f $INPUT_FILE_PATH ]; then
    aoc download --day $CURRENT_DAY -o --input-only --input-file ${INPUT_FILE_PATH}
    if [ $? -ne 0 ]; then exit 1; fi
# else
#     echo "Day ${CURRENT_DAY} already downloaded. Skipping download"
# fi

if [ ! -f $CURRENT_DAY_ANS_PATH ]; then
    cp $TEMPLATE_PATH $ANSWER_PATH
    sed -i "s/{{YEAR}}/$CURRENT_YEAR/g" $ANSWER_PATH
    sed -i "s/{{DAY}}/$CURRENT_DAY/g" $ANSWER_PATH
    cp $ANSWER_PATH $CURRENT_DAY_ANS_PATH
else
    cp $CURRENT_DAY_ANS_PATH $ANSWER_PATH
fi

cat <<EOT > .cargo/config.toml
[env]
CURRENT_YEAR = "$CURRENT_YEAR"
CURRENT_DAY_NAME = "$CURRENT_DAY_NAME"
CURRENT_DAY_ANS_PATH = "$CURRENT_DAY_ANS_PATH"
EOT

open "https://adventofcode.com/2023/day/${CURRENT_DAY#0}"
sleep 1
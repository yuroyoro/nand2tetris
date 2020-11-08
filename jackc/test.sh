#!/bin/bash
BASE=/Users/ozaki/dev/nand2tetris
PROJECT="${BASE}/projects"

tokenize() {
  local dir=$(echo "$1" | sed 's/ *$//')
  local name=$(basename $dir)

  echo "----------------------------------------------------------------------"
  echo "Tokenize ${name}"
  compile_out=$(cargo run -- --tokenize "${PROJECT}/${dir}" 2>&1)

  local status=$?
  if [ $status -ne 0 ]; then
    echo $compile_out
    exit $status
  fi

  for file in $(ls -1 ${PROJECT}/${dir}/*.tokens.xml); do
    local res="Pass"
    local class=$(basename $file | sed 's/.tokens.xml$//')

    diff_out=$(diff "${PROJECT}/${dir}/${class}.tokens.xml" "${PROJECT}/${dir}/${class}T.xml")

    if [ $? -ne 0 ]; then
      res="Fail"
    fi

    echo "Diff Result ${dir}/${class} : $res"

    RESULT+=("Tokenize : [${res}] ${PROJECT}/${dir}/${class}.jack")
  done;
}

parse() {
  local dir=$(echo "$1" | sed 's/ *$//')
  local name=$(basename $dir)

  echo "----------------------------------------------------------------------"
  echo "Parse ${name}"
  compile_out=$(cargo run -- --parse "${PROJECT}/${dir}" 2>&1)

  local status=$?
  if [ $status -ne 0 ]; then
    echo $compile_out
    exit $status
  fi

  for file in $(ls -1 ${PROJECT}/${dir}/*.ast.xml); do
    local res="Pass"
    local class=$(basename $file | sed 's/.ast.xml$//')

    diff_out=$(diff "${PROJECT}/${dir}/${class}.ast.xml" "${PROJECT}/${dir}/${class}.xml")

    if [ $? -ne 0 ]; then
      res="Fail"
    fi

    echo "Diff Result ${dir}/${class} : $res"

    RESULT+=("Parse : [${res}] ${PROJECT}/${dir}/${class}.jack")
  done;

}

tokenize "10/ArrayTest"
tokenize "10/ExpressionLessSquare"
tokenize "10/Square"

parse "10/ArrayTest"
parse "10/ExpressionLessSquare"
parse "10/Square"

echo "----------------------------------------------------------------------"
IFS=$'\n'
echo "${RESULT[*]}"
echo "----------------------------------------------------------------------"

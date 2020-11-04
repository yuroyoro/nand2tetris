#!/bin/bash
BASE=/Users/ozaki/dev/nand2tetris
PROJECT="${BASE}/projects"

tokenize() {
  local dir=$(echo "$1" | sed 's/ *$//')
  local name=$(basename $dir)

  echo "----------------------------------------------------------------------"
  echo "Tokenize ${name}"
  compile_out=$(cargo run tokenize "${PROJECT}/${dir}" 2>&1)

  local status=$?
  if [ $status -ne 0 ]; then
    echo $compile_out
    exit $status
  fi


  for file in $(ls -1 ${PROJECT}/${dir}/*T.result.xml); do
    local res="Pass"
    local class=$(basename $file | sed 's/T.result.xml$//')

    diff_out=$(diff "${PROJECT}/${dir}/${class}T.result.xml" "${PROJECT}/${dir}/${class}T.xml")

    if [ $? -ne 0 ]; then
      res="Fail"
    fi

    echo "Diff Result ${dir}/${class} : $res"

    RESULT+=("[${res}] ${PROJECT}/${dir}/${class}.jack")
  done;

}

tokenize "10/ArrayTest"
tokenize "10/ExpressionLessSquare"
tokenize "10/Square"

echo "----------------------------------------------------------------------"
IFS=$'\n'
echo "${RESULT[*]}"
echo "----------------------------------------------------------------------"

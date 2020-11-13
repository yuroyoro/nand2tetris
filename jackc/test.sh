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

    RESULT+=("Parse    : [${res}] ${PROJECT}/${dir}/${class}.jack")
  done;
}

compile() {
  local dir=$(echo "$1" | sed 's/ *$//')
  local name=$(basename $dir)
  local target=${PROJECT}/tmp/${name}

  echo "----------------------------------------------------------------------"
  echo "Compile ${name}"

  mkdir -p ${target}
  cp ${PROJECT}/${dir}/*.jack ${target}
  cp ${BASE}/tools/OS/*.vm ${target}

  local res="Pass"
  compile_out=$(cargo run -- "${target}")

  if [ $? -ne 0 ]; then
    echo $compile_out
    res="Fail"
  fi

  RESULT+=("Compile  : [${res}] ${target}")
}

tokenize "10/ArrayTest"
tokenize "10/ExpressionLessSquare"
tokenize "10/Square"

RESULT+=("----------------------------------------------------------------------")

parse "10/ArrayTest"
parse "10/ExpressionLessSquare"
parse "10/Square"

RESULT+=("----------------------------------------------------------------------")

mkdir -p ${PROJECT}/tmp/*
rm -rf ${PROJECT}/tmp/*

compile "10/ArrayTest"
compile "10/ExpressionLessSquare"
compile "10/Square"

compile "11/Seven"
compile "11/ConvertToBin"
compile "11/Square"
compile "11/Average"
compile "11/Pong"
compile "11/ComplexArrays"


echo "----------------------------------------------------------------------"
IFS=$'\n'
echo "${RESULT[*]}"
echo "----------------------------------------------------------------------"

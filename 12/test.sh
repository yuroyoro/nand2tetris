#!/bin/bash
BASE=/Users/ozaki/dev/nand2tetris
PROJECT="${BASE}/projects"
COMPILER="${PROJECT}/jackc/target/release/jackc"
VM_EMULATOR="${BASE}/tools/VMEmulator.sh"

compile() {
  local name=$1
  local depends1=$2
  local depends2=$3
  local depends3=$4

  local target=${PROJECT}/tmp/${name}

  rm -rf ${target}
  mkdir -p ${target}
  cp ${BASE}/tools/OS/*.vm ${target}
  cp ${PROJECT}/12/${name}Test/* ${target}
  cp "${PROJECT}/12/${name}.jack" ${target}

  if [ -e "${PROJECT}/12/${depends1}.jack" ]; then
    cp "${PROJECT}/12/${depends1}.jack" ${target}
  fi

  if [ -e "${PROJECT}/12/${depends2}.jack" ]; then
    cp "${PROJECT}/12/${depends2}.jack" ${target}
  fi

  if [ -e "${PROJECT}/12/${depends3}.jack" ]; then
    cp "${PROJECT}/12/${depends3}.jack" ${target}
  fi

  echo "----------------------------------------------------------------------"
  echo "Compile ${name}"
  compile_out=$($COMPILER "${target}" 2>&1)

  local status=$?
  if [ $status -ne 0 ]; then
    echo $compile_out
    exit $status
  fi

  RESULT+=("[Compile Ok] $1")
}

run_test() {
  local name=$1

  local target=${PROJECT}/tmp/${name}

  rm -rf ${target}
  mkdir -p ${target}
  cp ${BASE}/tools/OS/*.vm ${target}
  cp ${PROJECT}/12/${name}Test/* ${target}
  cp "${PROJECT}/12/${name}.jack" ${target}

  echo "----------------------------------------------------------------------"
  echo "Compile ${name}"
  compile_out=$($COMPILER "${target}" 2>&1)

  local status=$?
  if [ $status -ne 0 ]; then
    echo $compile_out
    exit $status
  fi

  echo "Run Test ${target}/${name}Test.tst"
  local res="Pass"
  test_out=$($VM_EMULATOR "${target}/${name}Test.tst" 2>&1)

  if [ $? -ne 0 ]; then
    res="Fail"

    cat ${target}/${name}Test.cmp ${target}/${name}Test.out
  fi

  RESULT+=("[${res}] $1 : ${test_out}")
}

pong() {
  local target=${PROJECT}/tmp/Pong

  rm -rf ${target}
  mkdir -p ${target}
  cp ${PROJECT}/12/*.jack ${target}
  cp ${PROJECT}/11/Pong/*.jack ${target}

  echo "----------------------------------------------------------------------"
  echo "Compile Pong"
  compile_out=$($COMPILER "${target}" 2>&1)

  local status=$?
  if [ $status -ne 0 ]; then
    echo $compile_out
    exit $status
  fi

  RESULT+=("[Compile Ok] Pong")
}

run_test "Math"
run_test "Array"
run_test "Memory"
compile "String" "Math"
compile "Screen"
compile "Output"
compile "Keyboard"
compile "Sys"
pong

echo "----------------------------------------------------------------------"
IFS=$'\n'
echo "${RESULT[*]}"
echo "----------------------------------------------------------------------"

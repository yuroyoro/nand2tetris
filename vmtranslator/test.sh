#!/bin/bash
BASE=/Users/ozaki/dev/nand2tetris
PROJECT="${BASE}/projects"
CPU_EMULATOR="${BASE}/tools/CPUEmulator.sh"

RESULT=()

run() {
  local dir=$(echo "$1" | sed 's/ *$//')
  local name=$(basename $dir)

  echo "----------------------------------------------------------------------"
  echo "Compile ${dir}/${name}.vm"
  compile_out=$(cargo run "${PROJECT}/${dir}/${name}.vm" 2>&1)

  local status=$?
  if [ $status -ne 0 ]; then
    echo $compile_out
    exit $status
  fi

  local res="Pass"

  echo "Run Test ${dir}/${name}.tst"
  test_out=$($CPU_EMULATOR "${PROJECT}/${dir}/${name}.tst")
  if [ $? -ne 0 ]; then
    res="Fail"
  fi

  RESULT+=("[${res}] $1 : ${test_out}")
}

rundir() {
  local dir=$(echo "$1" | sed 's/ *$//')
  local name=$(basename $dir)

  echo "----------------------------------------------------------------------"
  echo "Compile ${dir}"
  compile_out=$(cargo run "${PROJECT}/${dir}" 2>&1)

  local status=$?
  if [ $status -ne 0 ]; then
    echo $compile_out
    exit $status
  fi

  local res="Pass"

  echo "Run Test ${dir}/${name}.tst"
  test_out=$($CPU_EMULATOR "${PROJECT}/${dir}/${name}.tst")
  if [ $? -ne 0 ]; then
    res="Fail"
  fi

  RESULT+=("[${res}] $1 : ${test_out}")
}

run "07/StackArithmetic/SimpleAdd     "
run "07/StackArithmetic/StackTest     "
run "07/MemoryAccess/BasicTest        "
run "07/MemoryAccess/PointerTest      "
run "07/MemoryAccess/StaticTest       "
run "07/MemoryAccess/StaticTest       "
run "08/ProgramFlow/BasicLoop         "
run "08/ProgramFlow/FibonacciSeries   "
run "08/FunctionCalls/SimpleFunction  "
rundir "08/FunctionCalls/NestedCall      "
rundir "08/FunctionCalls/FibonacciElement"
rundir "08/FunctionCalls/StaticsTest     "

echo "----------------------------------------------------------------------"
IFS=$'\n'
echo "${RESULT[*]}"
echo "----------------------------------------------------------------------"

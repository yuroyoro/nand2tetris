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

  echo "Run Test ${dir}/${name}.vm"
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
# run "08/FunctionCalls/SimpleFunction  "
# run "08/FunctionCalls/NestedCall      "
# run "08/FunctionCalls/FibonacciElement"
# run "08/FunctionCalls/StaticTest      "

echo "----------------------------------------------------------------------"
IFS=$'\n'
echo "${RESULT[*]}"
echo "----------------------------------------------------------------------"

#!/bin/bash
BASE=/Users/ozaki/dev/nand2tetris
PROJECT="${BASE}/projects"
CHAPTOR="${PROJECT}/07"
CPU_EMULATOR="${BASE}/tools/CPUEmulator.sh"

echo "----------------------------------------------------------------------"
echo "Compile ${CHAPTOR}/StackArithmetic/SimpleAdd/SimpleAdd.vm"
cargo run "${CHAPTOR}/StackArithmetic/SimpleAdd/SimpleAdd.vm" > /dev/null

echo ""

echo "Run test ${CHAPTOR}/StackArithmetic/SimpleAdd/SimpleAdd.tst"
RESULT1=$($CPU_EMULATOR "${CHAPTOR}/StackArithmetic/SimpleAdd/SimpleAdd.tst")
echo $RESULT1

echo ""

echo "----------------------------------------------------------------------"
echo "Compile ${CHAPTOR}/StackArithmetic/StackTest/StackTest.vm"
cargo run "${CHAPTOR}/StackArithmetic/StackTest/StackTest.vm" > /dev/null

echo ""

echo "Run test ${CHAPTOR}/StackArithmetic/StackTest/StackTest.tst"
RESULT2=$($CPU_EMULATOR "${CHAPTOR}/StackArithmetic/StackTest/StackTest.tst")
echo $RESULT2

echo ""

echo "----------------------------------------------------------------------"
echo "Compile ${CHAPTOR}/MemoryAccess/BasicTest/BasicTest.vm"
cargo run "${CHAPTOR}/MemoryAccess/BasicTest/BasicTest.vm" > /dev/null

echo ""

echo "Run test ${CHAPTOR}/MemoryAccess/BasicTest/BasicTest.tst"
RESULT3=$($CPU_EMULATOR "${CHAPTOR}/MemoryAccess/BasicTest/BasicTest.tst")
echo $RESULT3

echo ""

echo "----------------------------------------------------------------------"
echo "Compile ${CHAPTOR}/MemoryAccess/PointerTest/PointerTest.vm"
cargo run "${CHAPTOR}/MemoryAccess/PointerTest/PointerTest.vm" > /dev/null

echo ""

echo "Run test ${CHAPTOR}/MemoryAccess/PointerTest/PointerTest.tst"
RESULT4=$($CPU_EMULATOR "${CHAPTOR}/MemoryAccess/PointerTest/PointerTest.tst")
echo $RESULT4

echo ""

echo "----------------------------------------------------------------------"
echo "Compile ${CHAPTOR}/MemoryAccess/StaticTest/StaticTest.vm"
cargo run "${CHAPTOR}/MemoryAccess/StaticTest/StaticTest.vm" > /dev/null

echo ""

echo "Run test ${CHAPTOR}/MemoryAccess/StaticTest/StaticTest.tst"
RESULT5=$($CPU_EMULATOR "${CHAPTOR}/MemoryAccess/StaticTest/StaticTest.tst")
echo $RESULT5

echo ""

echo "----------------------------------------------------------------------"
echo "StackArithmetic/SimpleAdd : ${RESULT1}"
echo "StackArithmetic/StackTest : ${RESULT2}"
echo "MemoryAccess/BasicTest    : ${RESULT3}"
echo "MemoryAccess/PointerTest  : ${RESULT4}"
echo "MemoryAccess/StaticTest   : ${RESULT5}"

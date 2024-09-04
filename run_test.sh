#!/usr/bin/env bash
TEST_CASE_ROOT="./test-cases"
TARGET_BIN="./target/debug/capnpc-angy"

files="${TEST_CASE_ROOT}/*"
cargo build

if [ -n $1 ]; then
  TEST_CASE=$1
  before_file=${TEST_CASE_ROOT}/${TEST_CASE}/${TEST_CASE}-before.capnp
  after_file=${TEST_CASE_ROOT}/${TEST_CASE}/${TEST_CASE}-after.capnp
  if [ ! -f ${before_file} ]; then
    exit 0
  fi
  if [ ! -f ${after_file} ]; then
    exit 0
  fi
  ${TARGET_BIN} ${before_file} ${after_file}
  exit 0
fi

for filepath in $files; do
  TEST_CASE=$(basename $filepath)
  before_file=${TEST_CASE_ROOT}/${TEST_CASE}/${TEST_CASE}-before.capnp
  after_file=${TEST_CASE_ROOT}/${TEST_CASE}/${TEST_CASE}-after.capnp
  if [ ! -f ${before_file} ]; then
    continue
  fi
  if [ ! -f ${after_file} ]; then
    continue
  fi
  ${TARGET_BIN} ${before_file} ${after_file}
done
function urldecode() { : "${*//+/ }"; echo -e "${_//%/\\x}"; }
timeout --signal=KILL ${TIMEOUT} gcc -o main src/main.c && timeout --signal=KILL ${TIMEOUT} ./main $(urldecode "${ARGS}")
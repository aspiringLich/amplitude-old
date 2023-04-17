function urldecode() { : "${*//+/ }"; echo -e "${_//%/\\x}"; }
timeout --signal=KILL ${TIMEOUT} python ./main.py $(urldecode "${ARGS}")
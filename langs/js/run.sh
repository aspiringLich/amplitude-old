function urldecode() { : "${*//+/ }"; echo -e "${_//%/\\x}"; }
timeout --signal=KILL ${TIMEOUT} node -e "$(cat main.js)" $(urldecode "${ARGS}")
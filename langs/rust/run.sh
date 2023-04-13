function urldecode() { : "${*//+/ }"; echo -e "${_//%/\\x}"; }
timeout --signal=KILL ${TIMEOUT} /usr/local/cargo/bin/cargo run -- $(urldecode "${ARGS}")
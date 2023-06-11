function urldecode() { : "${*//+/ }"; echo -e "${_//%/\\x}"; }
timeout --signal=KILL ${TIMEOUT} javac Main.java
timeout --signal=KILL ${TIMEOUT} java -cp . Main $(urldecode "${ARGS}")
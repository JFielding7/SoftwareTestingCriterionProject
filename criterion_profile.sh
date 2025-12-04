#!/usr/bin/env bash
set -e

GROUP_NAME="connect4_simple_profiler"
FUNC_NAME="evaluate_position"
REPORT_DIR="target/criterion/${GROUP_NAME}/${FUNC_NAME}/report"
FLAMEGRAPH_SVG="flamegraph.svg"
HTML_FILE="${REPORT_DIR}/index.html"

mkdir -p "$REPORT_DIR"

mv "$FLAMEGRAPH_SVG" "$REPORT_DIR/"

awk -v svg="$FLAMEGRAPH_SVG" '
  BEGIN { added=0 }
  /<tr>/ && added==0 { print; next_tr=1; next }
  next_tr==1 && /<\/tr>/ {
    print "    <td>"
    print "        <a href=\"" svg "\">"
    print "            <img src=\"" svg "\" alt=\"Flamegraph\" width=\"450\" height=\"300\" />"
    print "        </a>"
    print "    </td>"
    added=1
    next_tr=0
  }
  { print }
' "$HTML_FILE" > "$HTML_FILE.tmp"

mv "$HTML_FILE.tmp" "$HTML_FILE"

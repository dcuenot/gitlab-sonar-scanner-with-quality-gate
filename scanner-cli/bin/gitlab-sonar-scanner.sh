#!/bin/bash

set -euo pipefail

declare -a args

add_env_var_as_env_prop() {
  if [ "$1" ]; then
    args+=("-D$2=$1")
  fi
}


if [ -z "${SONAR_URL}" ]; then
  echo "Undefined \"SONAR_URL\" env" && exit 1
fi

add_env_var_as_env_prop "${SONAR_URL}" "sonar.login"
add_env_var_as_env_prop "${SONAR_LOGIN}" "sonar.login"
add_env_var_as_env_prop "${SONAR_PASSWORD}" "sonar.password"
add_env_var_as_env_prop "${SONAR_USER_HOME}" "sonar.userHome"
add_env_var_as_env_prop "${SONAR_PROJECT_BASE_DIR}" "sonar.projectBaseDir"

PROJECT_BASE_DIR="$PWD"
if [ "${SONAR_PROJECT_BASE_DIR:-}" ]; then
  PROJECT_BASE_DIR="${SONAR_PROJECT_BASE_DIR}"
fi
export SONAR_USER_HOME="$PROJECT_BASE_DIR/.sonar"

echo "${args[@]}"

#sonar-scanner "${args[@]}"
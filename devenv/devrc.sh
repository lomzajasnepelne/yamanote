start_devenv () {
  local python_venv=/root/venv

  OLD_PS1=$PS1 && \
  . ${python_venv}/bin/activate && \
  PS1=$OLD_PS1

  export PROJ_ROOT=~/yamanote
  export PYTHONPATH=${PROJ_ROOT}:${PYTHONPATH}

  echo "Yamanote development environment is ready"
}

start_devenv

unset -f start_devenv

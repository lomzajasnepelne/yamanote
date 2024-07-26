ARG DIGEST=sha256:2e863c44b718727c860746568e1d54afd13b2fa71b160f5cd9058fc436217b30
FROM ubuntu@${DIGEST}

RUN apt-get update && apt-get install -y \
  python3-full

RUN python3 -m venv /root/venv
COPY pip/requirements.txt /tmp/requirements.txt
COPY devrc.sh /tmp/devrc.sh
RUN . /root/venv/bin/activate && \
  pip3 install -r /tmp/requirements.txt
RUN cat /tmp/devrc.sh >> /root/.bashrc
RUN rm /tmp/devrc.sh /tmp/requirements.txt

RUN mkdir /root/yamanote
WORKDIR "/root/yamanote"

ENTRYPOINT [ "/bin/bash" ]

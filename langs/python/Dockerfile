FROM python:latest

RUN useradd -m runner -d /runner
RUN usermod -p '!!' root

USER runner
WORKDIR /runner

ADD --chown=runner run.sh .

RUN pip install --upgrade pip && pip install toml

ENTRYPOINT ["bash", "./run.sh"]
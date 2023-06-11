FROM gcc:latest

RUN useradd -m runner -d /runner
RUN usermod -p '!!' root

USER runner
WORKDIR /runner

ADD --chown=runner run.sh .

CMD ["bash", "./run.sh"]
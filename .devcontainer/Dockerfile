FROM eelectronn7998/dev-containers:fedora

RUN dnf update -y
RUN dnf upgrade -y

RUN sudo dnf install make automake gcc gcc-c++ kernel-devel -y
RUN git config --global --add safe.directory '*'

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

FROM ubuntu:16.04

RUN useradd -m -s /bin/bash nonroot

WORKDIR /app

COPY calculator /app
COPY socat /app

RUN touch users.dat
RUN mkdir calc_history

RUN chmod 500 /app/calculator /app/socat && chown nonroot:nonroot /app/calculator /app/socat
RUN chmod -R 777 /app/calc_history && chown -R nonroot:nonroot /app/calc_history
RUN chmod -R 777 /app/users.dat && chown nonroot:nonroot /app/users.dat

USER nonroot


CMD ["sh", "-c", "while true; do /app/socat -T 10 TCP-LISTEN:1234,reuseaddr,fork,nodelay,rcvbuf=1048576,sndbuf=1048576 EXEC:'/app/calculator',pty,raw,echo=0; done"]


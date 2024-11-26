FROM python:3.11

RUN apt-get update && apt-get install -y ncat unzip wget


WORKDIR /app

RUN mkdir users

COPY start.sh .
COPY main.py .


RUN chmod +x main.py
RUN chmod +x start.sh

CMD ["bash", "start.sh"]

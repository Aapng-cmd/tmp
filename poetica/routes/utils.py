import os
import secrets
import sys
from hashlib import sha1
from binascii import hexlify, unhexlify

secret_key_file_path = "../secret_key.txt"


def get_secret_key():
    if os.path.exists(secret_key_file_path):
        with open(secret_key_file_path, 'r') as file:
            secret_key = file.read().strip()
    else:
        secret_key = secrets.token_hex(32)
        with open(secret_key_file_path, 'w') as file:
            file.write(secret_key)

    return secret_key


secret_key = get_secret_key()


def hash_(a):
    return sha1(secret_key.encode() + a).hexdigest()


def get_session_from_username(username):
    for_session = username + '|' + hash_(username.encode())

    return hexlify(for_session.encode()).decode()


def get_username_from_session(session):
    try:
        parts = unhexlify(session.encode())
        data, sym, sign = parts.rpartition(b'|')

        is_verified = hash_(data) == sign.decode()

        username = parts.split(b'|')[-2].decode()

        return username, is_verified

    except:
        return "", False

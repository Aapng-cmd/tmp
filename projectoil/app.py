import os
import secrets

from flask import Flask
from flask_sqlalchemy import SQLAlchemy
import random
import secrets

app = Flask(__name__)

app.config['UPLOAD_FOLDER'] = 'static'
app.config['MAX_CONTENT_LENGTH'] = 1024


def generate_and_save_secret_key(filepath='secret_key.txt', seed=31337):
    random.seed(seed)

    secret_key = random.randbytes(16)

    with open(filepath, 'w') as f:
        f.write(secret_key.hex())


generate_and_save_secret_key()

with open('secret_key.txt', 'r') as f:
    app.config['SECRET_KEY'] = f.read().strip()

DB_USER = os.getenv('DB_USER')
DB_PASSWORD = os.getenv('DB_PASSWORD')
DB_NAME = os.getenv('DB_NAME')

DATABASE_URL = f'postgresql://{DB_USER}:{DB_PASSWORD}@postgres:5432/{DB_NAME}'
app.config['SQLALCHEMY_DATABASE_URI'] = DATABASE_URL

db = SQLAlchemy(app)

from models import *
from routes import register_routes

register_routes(app)

with app.app_context():
    db.create_all()


if __name__ == '__main__':
    app.run(debug=False)

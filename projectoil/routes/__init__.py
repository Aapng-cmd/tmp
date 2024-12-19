from flask import request, redirect, url_for, flash, session
from .auth import auth_bp
from .users import users_bp
from .teams import teams_bp
from models import User

from app import app


@app.route('/')
def index():
    if 'username' not in session:
        return redirect(url_for('auth.login'))

    username = session['username']
    user = User.query.filter_by(username=username).first()
    if not user:
        return redirect(url_for('auth.login'))

    return redirect(url_for('users.profile', username=user.username))


def register_routes(app):
    app.register_blueprint(auth_bp)
    app.register_blueprint(users_bp)
    app.register_blueprint(teams_bp)

from flask import request, redirect, url_for, flash

from .auth import auth_bp
from .users import users_bp
from .poems import poems_bp

from app import app
from models import User
from .utils import get_username_from_session


@app.route('/')
def index():
    try:
        session_id = request.cookies.get('session_id')
        if not session_id:
            return redirect(url_for('auth.login'))

        username, is_verified = get_username_from_session(session_id)
        if not is_verified:
            flash('Invalid session. Please log in again.', 'error')
            return redirect(url_for('auth.login'))

        user = User.query.filter_by(username=username).first()
        if not user:
            return redirect(url_for('auth.login'))

        return redirect(url_for('users.profile', username=username))
    except Exception as e:
        return f'An error occurred: {str(e)}'


def register_routes(app):
    app.register_blueprint(auth_bp)
    app.register_blueprint(users_bp)
    app.register_blueprint(poems_bp)
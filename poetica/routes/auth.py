import secrets

from flask import Blueprint
from app import db
from models import User
import uuid
from flask import render_template, request, redirect, url_for, flash, make_response
from .utils import get_session_from_username

auth_bp = Blueprint('auth', __name__)

secret_key = secrets.token_hex(32)


@auth_bp.route('/register', methods=['GET', 'POST'])
def register():
    try:
        if request.method == 'POST':
            username = request.form.get('username')
            password = request.form.get('password')
            confirm_password = request.form.get('confirm_password')

            if len(password) < 8:
                flash('Password is too short', 'error')
                return redirect(url_for('auth.register'))

            if password != confirm_password:
                flash('Passwords do not match!', 'error')
                return redirect(url_for('auth.register'))

            existing_user = User.query.filter_by(username=username).first()
            if existing_user:
                flash('Username already taken', 'error')
                return redirect(url_for('auth.register'))

            csrf_token = str(uuid.uuid4())

            new_user = User(username=username, password=password, csrf_token=csrf_token)
            db.session.add(new_user)
            db.session.commit()

            flash('Your account has been created! You can now log in.', 'success')
            return redirect(url_for('auth.login'))

        return render_template('register.html')

    except Exception as e:
        db.session.rollback()
        flash(f'An error occurred: {str(e)}', 'error')
        return redirect(url_for('auth.register'))


@auth_bp.route('/login', methods=['GET', 'POST'])
def login():
    try:
        if request.method == 'POST':
            username = request.form.get('username')
            password = request.form.get('password')

            user = User.query.filter_by(username=username).first()

            if user and user.password == password:
                user_session = get_session_from_username(username.strip())

                response = make_response(redirect(url_for('users.profile', username=user.username)))
                response.set_cookie('session_id', user_session, max_age=60 * 60 * 24 * 30)
                response.set_cookie('csrf_token', user.csrf_token, max_age=60 * 60 * 24 * 30)

                return response

            flash('Login unsuccessful. Please check your username and password.', 'error')
            return redirect(url_for('auth.login'))

        return render_template('login.html')

    except Exception as e:
        flash(f'An error occurred: {str(e)}', 'error')
        return redirect(url_for('auth.login'))


@auth_bp.route('/logout')
def logout():
    try:
        response = make_response(redirect(url_for('auth.login')))
        response.delete_cookie('session_id')
        response.delete_cookie('csrf_token')

        flash('You have been logged out!', 'success')
        return response
    except Exception as e:
        flash(f'An error occurred: {str(e)}', 'error')
        return redirect(url_for('auth.login'))
import logging

from flask import Blueprint
from app import db
from models import User
from flask import render_template, request, redirect, url_for, flash, make_response, session

auth_bp = Blueprint('auth', __name__)

logging.basicConfig(level=logging.ERROR)


@auth_bp.route('/register', methods=['GET', 'POST'])
def register():
    try:
        if request.method == 'POST':
            username = request.form.get('username')
            password = request.form.get('password')
            confirm_password = request.form.get('confirm_password')

            if len(password) < 8:
                return "Слишком короткий пароль"

            if password != confirm_password:
                return "Введенные пароли не совпадают"

            existing_user = User.query.filter_by(username=username).first()
            if existing_user:
                return "Такой пользователь уже существует"

            new_user = User(username=username, password=password)
            db.session.add(new_user)
            db.session.commit()

            return redirect(url_for('auth.login'))

        return render_template('register.html')

    except Exception as e:
        db.session.rollback()
        logging.error(f"Ошибка при регистрации пользователя: {e}")
        return redirect(url_for('auth.register'))


@auth_bp.route('/login', methods=['GET', 'POST'])
def login():
    try:
        if request.method == 'POST':
            username = request.form.get('username')
            password = request.form.get('password')

            user = User.query.filter_by(username=username).first()

            if not user or password != user.password:
                return "Неверное имя пользователя или пароль"

            session['username'] = user.username
            return redirect(url_for('users.profile', username=user.username))

        return render_template('login.html')

    except Exception as e:
        db.session.rollback()
        logging.error(f"Ошибка при входе пользователя: {e}")
        return redirect(url_for('auth.login'))


@auth_bp.route('/logout')
def logout():
    session.pop('username', None)
    return redirect(url_for('auth.login'))
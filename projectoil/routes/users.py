import logging

from flask import Blueprint
from app import db
from models import User, Team
from flask import render_template, request, redirect, url_for, make_response, session, jsonify

users_bp = Blueprint('users', __name__)

logging.basicConfig(level=logging.ERROR)


@users_bp.route('/profile/<username>', methods=['GET', 'POST'])
def profile(username):
    try:
        if 'username' not in session:
            return redirect(url_for('auth.login'))

        session_username = session['username']
        user = User.query.filter_by(username=session_username).first()
        if not user:
            return redirect(url_for('auth.login'))

        user = User.query.filter_by(username=username).first()
        if not user:
            return "Такой пользователь не найден"

        user_teams = user.teams

        if request.method == 'POST' and session_username == user.username:
            if user.private_info:
                return "Информация уже установлена"

            private_info = request.form.get('private_info')
            user.private_info = private_info
            db.session.commit()
            return redirect(url_for('users.profile', username=user.username))

        return render_template('profile.html', user=user, session_username=session_username, teams=user_teams)

    except Exception as e:
        db.session.rollback()
        logging.error(f"Ошибка в профиле пользователя: {e}")
        return redirect(url_for('auth.login'))


@users_bp.route('/users/recent', methods=['GET'])
def get_recent_users():
    try:
        users = User.query.order_by(User.id.desc()).limit(500).all()

        usernames = [user.username for user in users]

        return jsonify({"usernames": usernames}), 200

    except Exception as e:
        db.session.rollback()
        logging.error(f"Ошибка при получении пользователей: {e}")
        return redirect(url_for('auth.login'))


import logging
import os
import uuid
from binascii import unhexlify

from flask import Blueprint, render_template, request, redirect, url_for, session, abort, Response
from app import db, app
from models import User, Team, Task

import base64

teams_bp = Blueprint('teams', __name__)

logging.basicConfig(level=logging.ERROR)

MAX_FILE_SIZE = 1024


@teams_bp.route('/create_team', methods=['GET', 'POST'])
def create_team():
    try:
        if 'username' not in session:
            return redirect(url_for('auth.login'))

        if request.method == 'POST':
            team_name = request.form.get('team_name')
            description = request.form.get('description')

            existing_team = Team.query.filter_by(name=team_name).first()
            if existing_team:
                return "Команда с таким именем уже существует"

            new_team = Team(name=team_name, description=description)
            current_user = User.query.filter_by(username=session['username']).first()

            new_team.members.append(current_user)
            db.session.add(new_team)
            db.session.commit()

            return redirect(url_for('users.profile', username=current_user.username))

        return render_template('create_team.html')

    except Exception as e:
        db.session.rollback()
        logging.error(f"Ошибка при создании команды: {e}")
        return redirect(url_for('auth.login'))


@teams_bp.route('/team/<int:team_id>', methods=['GET', 'POST'])
def view_team(team_id):
    try:
        if 'username' not in session:
            return redirect(url_for('auth.login'))

        current_user = User.query.filter_by(username=session['username']).first()
        if not current_user:
            return redirect(url_for('auth.login'))

        team = Team.query.get(team_id)
        if not team:
            return "Команда не найдена"

        if current_user not in team.members:
            return "У вас нет доступа к этой команде"

        if request.method == 'POST' and 'task_title' in request.form:
            task_title = request.form.get('task_title')
            task_description = request.form.get('task_description')
            new_task = Task(title=task_title, description=task_description, team=team)
            db.session.add(new_task)
            db.session.commit()
            return redirect(url_for('teams.view_team', team_id=team.id))

        if request.method == 'POST' and 'new_member_username' in request.form:
            new_member_username = request.form.get('new_member_username')
            new_member = User.query.filter_by(username=new_member_username).first()
            if new_member and new_member not in team.members:
                token = base64.b64encode(f"{new_member_username}|{team_id}".encode()).hex()
                return f"Передайте пользователю токен для вступления в команду: {token}"
            elif not new_member:
                return "Пользователь с таким именем не найден"
            else:
                return "Пользователь уже в команде"

        if request.method == 'POST' and 'file' in request.files:
            file = request.files['file']
            if len(file.read()) > MAX_FILE_SIZE:
                return 'Размер файла превышает допустимый лимит (1KB)'

            file.seek(0)
            file_id = str(uuid.uuid4())
            ext = file.filename.split('.')[1]
            filepath = os.path.join('static', f'{file_id}.{ext}')
            file.save(filepath)
            return f"Файл успешно сохранен в {filepath}"

        tasks = team.tasks
        members = team.members

        return render_template('view_team.html', team=team, tasks=tasks, members=members)

    except Exception as e:
        db.session.rollback()
        return f"Ошибка: {e}"


@teams_bp.route('/join_team', methods=['POST'])
def join_team():
    try:
        token = request.form['token']

        if 'username' not in session:
            return redirect(url_for('auth.login'))

        current_user = User.query.filter_by(username=session['username']).first()
        if not current_user:
            return redirect(url_for('auth.login'))

        team_id = base64.b64decode(unhexlify(token)).decode().split('|')[1]
        team = Team.query.filter_by(id=team_id).first()

        if team:
            if current_user not in team.members:
                team.members.append(current_user)
                db.session.commit()
                return redirect(url_for('users.profile', username=current_user.username))
            else:
                return "Вы уже состоите в этой команде"
        else:
            return "Неверный токен"

    except Exception as e:
        db.session.rollback()
        return f"Ошибка при присоединении к команде: {e}"


@teams_bp.route('/download')
def download_file():
    filename = request.args.get('filename', '')

    uploads_dir = app.config['UPLOAD_FOLDER']
    file_path = uploads_dir + "/" + filename

    if not os.path.exists(file_path):
        abort(404, description="Файл не найден")

    try:
        with open(file_path, 'rb') as file:
            file_content = file.read()
            return Response(file_content, content_type="application/octet-stream")
    except Exception as e:
        abort(500, description="Ошибка при чтении файла")
from flask import Blueprint, jsonify
from app import db
from models import User, Poem, PoemAccess, Comment
from flask import render_template, request, redirect, url_for, flash, make_response
from .utils import get_username_from_session

poems_bp = Blueprint('poems', __name__)


@poems_bp.route('/poems/create', methods=['GET', 'POST'])
def create():
    try:
        session_id = request.cookies.get('session_id')
        if not session_id:
            return redirect(url_for('auth.login'))

        username_from_session, is_verified = get_username_from_session(session_id)
        if not is_verified:
            flash('Invalid session. Please log in again.', 'error')
            return redirect(url_for('auth.login'))

        user = User.query.filter_by(username=username_from_session).first()
        if not user:
            flash('User not found.', 'error')
            return redirect(url_for('auth.login'))

        if request.method == 'POST':
            csrf_token = request.form.get('csrf_token')
            if not csrf_token or csrf_token != request.cookies.get('csrf_token'):
                return make_response(jsonify({"error": "Invalid CSRF token"}), 403)

            title = request.form.get('title')
            content = request.form.get('content')
            is_public = request.form.get('isPublic') == 'on'

            new_poem = Poem(title=title, content=content, user_id=user.id, isPublic=is_public)
            db.session.add(new_poem)
            db.session.commit()

            flash('Poem created successfully!', 'success')
            return redirect(url_for('users.profile', username=user.username))

        return render_template('create_poem.html', user=user, session_username=username_from_session,
                               session_csrf_token=user.csrf_token)
    except Exception as e:
        db.session.rollback()
        return make_response(jsonify({"error": f'An error occurred: {str(e)}'}), 500)


@poems_bp.route('/poems/<int:poem_id>', methods=['GET', 'POST'])
def view_poem(poem_id):
    try:
        session_id = request.cookies.get('session_id')
        if not session_id:
            return redirect(url_for('auth.login'))

        username_from_session, is_verified = get_username_from_session(session_id)
        if not is_verified:
            flash('Invalid session. Please log in again.', 'error')
            return redirect(url_for('auth.login'))

        user = User.query.filter_by(username=username_from_session).first()
        if not user:
            flash('User not found.', 'error')
            return redirect(url_for('auth.login'))

        poem = Poem.query.get(poem_id)
        if not poem:
            return make_response(jsonify({"error": "Poem not found"}), 404)

        user_has_access = poem.isPublic or poem.author == user or \
                          PoemAccess.query.filter_by(user_id=user.id, poem_id=poem.id).first() is not None

        if not user_has_access:
            return make_response(jsonify({"error": "Invalid access"}), 403)

        comments = Comment.query.filter_by(poem_id=poem_id).all()

        if request.method == 'POST' and 'comment_content' in request.form:
            csrf_token = request.form.get('csrf_token')
            if not csrf_token or csrf_token != request.cookies.get('csrf_token'):
                return make_response(jsonify({"error": "Invalid CSRF token"}), 403)

            comment_content = request.form.get('comment_content')
            new_comment = Comment(content=comment_content, poem_id=poem.id, user_id=user.id)
            db.session.add(new_comment)
            db.session.commit()

            flash('Comment added successfully!', 'success')
            return redirect(url_for('poems.view_poem', poem_id=poem.id))

        return render_template('view_poem.html', poem=poem, comments=comments,
                               session_username=username_from_session, session_csrf_token=user.csrf_token,
                               user_has_access=user_has_access)

    except Exception as e:
        db.session.rollback()
        return make_response(jsonify({"error": f'An error occurred: {str(e)}'}), 500)


@poems_bp.route('/poems/grant_access/<int:poem_id>', methods=['POST'])
def grant_access(poem_id):
    try:
        session_id = request.cookies.get('session_id')
        if not session_id:
            return redirect(url_for('auth.login'))

        username_from_session, is_verified = get_username_from_session(session_id)
        if not is_verified:
            flash('Invalid session. Please log in again.', 'error')
            return redirect(url_for('auth.login'))

        owner_username = request.form.get('owner_username')

        user = User.query.filter_by(username=owner_username).first()
        if not user:
            flash('User not found.', 'error')
            return redirect(url_for('auth.login'))

        poem = Poem.query.get(poem_id)
        if not poem:
            return make_response(jsonify({"error": "Poem not found"}), 404)

        csrf_token = request.form.get('csrf_token')
        if not csrf_token or csrf_token != request.cookies.get('csrf_token'):
            return make_response(jsonify({"error": "Invalid CSRF token"}), 403)

        grant_username = request.form.get('grant_username')

        if grant_username == owner_username:
            return make_response(jsonify({"error": "You cannot grant yourself"}), 400)

        grant_user = User.query.filter_by(username=grant_username).first()
        if not grant_user:
            return make_response(jsonify({"error": "User not found"}), 404)

        existing_access = PoemAccess.query.filter_by(user_id=grant_user.id, poem_id=poem.id).first()
        if existing_access:
            return make_response(jsonify({"error": "Already accessed"}), 400)

        new_access = PoemAccess(user_id=grant_user.id, poem_id=poem.id)
        db.session.add(new_access)
        db.session.commit()

        flash('Access granted successfully!', 'success')
        return redirect(url_for('poems.view_poem', poem_id=poem.id))

    except Exception as e:
        db.session.rollback()
        return make_response(jsonify({"error": f'An error occurred: {str(e)}'}), 500)

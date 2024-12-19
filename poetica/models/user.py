from app import db


class User(db.Model):
    __tablename__ = 'users'

    id = db.Column(db.Integer, primary_key=True)
    username = db.Column(db.String(50), unique=True, nullable=False)
    password = db.Column(db.String(50), nullable=False)
    isPrivate = db.Column(db.Boolean, default=False)
    aboutMe = db.Column(db.Text, nullable=True)
    csrf_token = db.Column(db.Text, nullable=False)

    poems = db.relationship('Poem', back_populates='author', lazy=True)
    comments = db.relationship('Comment', back_populates='user', lazy=True)
    poem_access = db.relationship('PoemAccess', back_populates='user', lazy=True)

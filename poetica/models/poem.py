from app import db


class Poem(db.Model):
    __tablename__ = 'poems'

    id = db.Column(db.Integer, primary_key=True)
    title = db.Column(db.String(100), nullable=False)
    content = db.Column(db.Text, nullable=False)
    isPublic = db.Column(db.Boolean, default=False)
    user_id = db.Column(db.Integer, db.ForeignKey('users.id'), nullable=False)

    author = db.relationship('User', back_populates='poems')
    comments = db.relationship('Comment', back_populates='poem', lazy=True)
    poem_access = db.relationship('PoemAccess', back_populates='accessed_poem', lazy=True)

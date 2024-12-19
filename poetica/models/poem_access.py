from app import db


class PoemAccess(db.Model):
    __tablename__ = 'poem_access'

    id = db.Column(db.Integer, primary_key=True)
    user_id = db.Column(db.Integer, db.ForeignKey('users.id'), nullable=False)
    poem_id = db.Column(db.Integer, db.ForeignKey('poems.id'), nullable=False)

    user = db.relationship('User', back_populates='poem_access')
    accessed_poem = db.relationship('Poem', back_populates='poem_access')

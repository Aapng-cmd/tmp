from app import db


class User(db.Model):
    __tablename__ = 'users'

    id = db.Column(db.Integer, primary_key=True)
    username = db.Column(db.String(50), unique=True, nullable=False)
    password = db.Column(db.String(50), nullable=False)
    private_info = db.Column(db.Text, nullable=True)

    teams = db.relationship('Team', secondary='team_members', back_populates='members')

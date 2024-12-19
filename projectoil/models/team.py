from app import db


class Team(db.Model):
    __tablename__ = 'teams'

    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(50), unique=True, nullable=False)
    description = db.Column(db.Text, nullable=True)

    members = db.relationship('User', secondary='team_members', back_populates='teams')
    tasks = db.relationship('Task', back_populates='team', lazy=True)

from db import connection_manager
from sqlite3 import DatabaseError
from .Beer import Beer


class Basket:
    def __init__(self, id, name, owner, cost, description, sent_to, beers):
        self.id = id
        self.name = name
        self.beers = beers
        self.cost = cost
        self.owner = owner
        self.sent_to = sent_to
        self.description = description

    @staticmethod
    def get_basket(row):
        beers = Beer.get_beers_from_basket(str(row[0]))
        return Basket(*row, beers=beers)

    @staticmethod
    def get_basket_by_id(id):
        query = "SELECT * FROM basket WHERE id = ?"
        res = connection_manager(query, id)
        if not res:
            return
        return Basket.get_basket(res[0])

    @staticmethod
    def get_user_baskets(username):
        query = "SELECT * FROM basket WHERE owner = ?"
        res = connection_manager(query, username)
        if not res:
            return
        return [Basket.get_basket(row) for row in res]

    @staticmethod
    def filter_baskets(filter, value, username):
        query = f"SELECT id, name, owner, cost, description, sent_to FROM basket WHERE owner = ? AND {filter} = ?"
        print(query)
        res = connection_manager(query, username, value)
        if not res:
            return
        return [Basket.get_basket(row) for row in res]

    @staticmethod
    def create_basket(name, owner, cost, description, beers):
        query = "INSERT INTO basket (name, owner, cost, description, sent_to) VALUES (?, ?, ?, ?, 'default') RETURNING id"
        res = connection_manager(query, name, owner, cost, description)[0]
        if not res:
            return
        Basket.add_beers_to_basket(beers, res[0])
        return "Success"

    @staticmethod
    def add_beers_to_basket(beers_id, basket_id):
        query = "INSERT INTO beers_basket VALUES "
        for id in beers_id:
            query += f" (%d, %d)," % (int(basket_id), int(id))
        connection_manager(query[:-1])

    @staticmethod
    def send_basket(basket_id, user_to, user_from):
        query = "UPDATE basket SET sent_to = ? WHERE id=? and owner=? RETURNING 1"
        connection_manager(query, user_to, basket_id, user_from)

    @staticmethod
    def get_given_baskets(user_from, user_to):
        if user_from:
            query = "SELECT * FROM basket WHERE owner = ? and sent_to = ?"
            res = connection_manager(query, user_from, user_to)
            if not res:
                return
        else:
            query = "SELECT * FROM basket WHERE sent_to = ?"
            res = connection_manager(query, user_to)
            if not res:
                return
        return [Basket.get_basket(row) for row in res]

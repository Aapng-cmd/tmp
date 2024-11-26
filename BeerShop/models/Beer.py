from db import connection_manager
from sqlite3 import DatabaseError


class Beer:
    def __init__(self, id, name, color, cost):
        self.id = id
        self.name = name
        self.color = color
        self.cost = cost

    @staticmethod
    def get_beer_by_id(id):
        query = "SELECT * FROM beers WHERE id = ?"
        res = connection_manager(query, str(id))
        if not res:
            return
        return Beer(*res[0])

    @staticmethod
    def get_all_beers():
        query = "SELECT * FROM beers"
        res = connection_manager(query)
        if not res:
            return
        return [Beer(*o) for o in res]

    @staticmethod
    def get_beers_from_basket(basket_id):
        query = "SELECT beer_id FROM beers_basket WHERE basket_id = ?"
        res = connection_manager(query, basket_id)
        if not res:
            return
        return [Beer.get_beer_by_id(id[0]) for id in res]

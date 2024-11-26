from models.User import User
from models.Basket import Basket


def validate_beers(beer_ids):
    for id in beer_ids:
        if not id.isdigit():
            return True
    return False


def if_exists(username, basket_id):
    user = User.get_by_username(username)
    basket = Basket.get_basket_by_id(basket_id)
    return user and basket

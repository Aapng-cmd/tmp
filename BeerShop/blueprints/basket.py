from flask import Blueprint
from flask_login import LoginManager, login_required, current_user
from flask import Flask, request, render_template, redirect, url_for, redirect
from models.Basket import Basket
from models.Beer import Beer
from helpers.calculate_cost import calc_basket_cost
from helpers.validators import validate_beers, if_exists

basket = Blueprint("basket routes", __name__)


@basket.route("/basket/create", methods=["GET", "POST"])
@login_required
def create_basket():
    user = current_user
    if not user:
        return redirect("/login")
    if request.method == "GET":
        beers = Beer.get_all_beers()
        return render_template("create-basket.html", beers=beers)
    form = request.form
    name = form.get("name", "")
    beer_ids = form.getlist("beers")
    if validate_beers(beer_ids):
        return render_template(
            "create-basket.html", error="Beers id have to be a number"
        )
    description = form.get("description", "")
    if not (name and beer_ids and description):
        return render_template(
            "create-basket.html", error="All fields have to be filled"
        )
    cost = calc_basket_cost(beer_ids)
    Basket.create_basket(name, user.username, cost, description, beer_ids)
    return redirect("/basket/filter")


@basket.route("/basket/send", methods=["POST"])
@login_required
def send_basket():
    user = current_user
    if not user:
        return redirect("/login")
    user_from = user.username
    form = request.form
    user_to = form.get("user_to", "")
    basket_id = form.get("basket_id", "")
    if not if_exists(user_to, basket_id):
        return redirect('/basket/filter?error="User or basket doesn\'t exists"')
    Basket.send_basket(basket_id, user_to, user_from)
    return redirect("/basket/filter")


@basket.route("/basket/given", methods=["GET"])
@login_required
def get_given_baskets():
    user = current_user
    if not user:
        return redirect("/login")
    params = request.args
    user_from = params.get("user_from", "")
    user_to = params.get("user_to")
    if not user_to:
        user_to = user.username
    baskets = Basket.get_given_baskets(user_from, user_to)
    return render_template("baskets.html", baskets=baskets)


@basket.route("/basket/filter", methods=["GET"])
@login_required
def sort_basket():
    user = current_user
    if not user:
        return redirect("/login")
    params = request.args
    filter_by = params.get("field", "")
    value = params.get("value", "")
    error = params.get("error", "")
    if not (filter_by and value):
        baskets = Basket.get_user_baskets(user.username)
        return render_template("filter.html", baskets=baskets, error=error)
    baskets = Basket.filter_baskets(filter_by, value, user.username)
    return render_template(
        "filter.html", baskets=baskets, field=filter_by, value=value, error=error
    )

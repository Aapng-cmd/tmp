#!/usr/local/bin/python

import hashlib
import csv
import os

def register():
    username = input("Enter your username: ")

    # Hash the password
    md5 = hashlib.md5()
    sha256 = hashlib.sha256()
    md5.update(username.encode('utf-8'))
    hashed_password = sha256.hexdigest()[:20]
    
    # Check if the username already exists
    with open('passwords.csv', mode='r') as file:
        reader = csv.reader(file)
        for row in reader:
            if row[0] == username:
                print("This username is already registered. Please choose a different username.")
                return
    
    # Save the username and hashed password to a CSV file
    with open('passwords.csv', mode='a', newline='') as file:
        writer = csv.writer(file)
        writer.writerow([username, hashed_password])
    
    print(f"You are all set! Your password is: {hashed_password}")

def login(password):
    
    
    # Check if the password exists for any user
    with open('passwords.csv', mode='r') as file:
        reader = csv.reader(file)
        for row in reader:
            if row[1] == hashed_password:
                print("Welcome back!")
                return True
    print("Invalid password.")
    return False

def write_note(note):
    # Save the note to a CSV file
    filename = "notes.csv"
    
    # Count existing notes to determine the note number
    note_number = 0
    if os.path.exists(filename):
        with open(filename, mode='r') as file:
            reader = csv.reader(file)
            note_number = sum(1 for _ in reader)  # Count existing notes
    
    # Append the new note
    with open(filename, mode='a', newline='') as file:
        writer = csv.writer(file)
        writer.writerow([note])
    
    # Print the note number (1-based index)
    print(note_number - 1)

def read_notes():
    filename = "notes.csv"
    if not os.path.exists(filename):
        print("No notes found.")
        return
    
    with open(filename, mode='r') as file:
        reader = csv.reader(file)
        notes = list(reader)
        if not notes:
            print("No notes found.")
            return
        
    return notes  # Return the notes for further processing

def read_note_by_id(note_id):
    notes = read_notes()
    if notes is None or note_id < 0 or note_id > len(notes) - 1:
        print("Invalid note ID.")
        return

    # Print the specific note by ID (1-based index)
    print(f"{notes[note_id][0]}")

def main():
    print("""Welcome to PunchNotes v0.3
Please select an option:
1. Login
2. Register
>""")
    choice = input()
    if choice not in ["1", "2"]:
        return

    if choice == "2":
        register()
    
    password = input("Please enter your password: ")
    if not login(password):
        return
    
    while True:
        print("Hi!")
        print("""1. Write Note
2. Read Note
""")
        choice = input(">")
        if choice == "1":
            note = input("Type your note here: ")
            write_note(note)
        elif choice == "2":
            note_id = int(input("Enter the ID of the note you'd like to read: "))
            read_note_by_id(note_id)
        else:
            print("Invalid choice. Please try again.")

if __name__ == "__main__":
    main()

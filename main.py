#/usr/bin/env python3


import customtkinter as ctk
import json
from argparse import ArgumentParser, Namespace


INICIO = [
    "Limites",
    "Fraquezas",
    "Necessidades",
    "Carencias",
    "Objetivos",
]

class App:
    def __init__(self, master: ctk.Ctk):
        self.master = master
        self.master.geometry('250x200')
        self.master.resizable(False, False)
        self.master.wm_title('PVP')


def banco_de_dados():
    try:
        with open('pvp.json') as file:
            arquivo = json.load(file)
            return arquivo
    except json.JSONDecodeError as error:
        with open('pvp.json', 'w') as file:
            arquivo = json.dumps(INICIO, indent=4)
            file.write(arquivo)
            arquivo = json.loads(file)
            return arquivo


def banner():
    banner_str = """
    add categorie tiltulo [como_estou, como_devo_esta, como_farei, que_meios_usarei, quando_farei]
    remove categorie id_item
    """


def add(db: dict, item: str, texto: str) -> dict:
   db[item] = texto
   return db


def remove(db: dict, item: str, texto: str) -> dict:
    ...


def main():
    db = banco_de_dados()


parser = ArgumentParser()

parser.add_argument('add', help='add texto')

args: Namespace = parser.parse_args()


if __name__=="__main__":
    app = ctk.Ctk()
    gui = App(master=app)
    app.mainloop()